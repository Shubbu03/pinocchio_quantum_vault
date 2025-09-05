use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use solana_winternitz::signature::WinternitzSignature;

use crate::states::{load_ix_data, DataLen};

#[repr(C)]
pub struct SplitVault {
    pub signature: WinternitzSignature,
    pub amount: u8,
    pub bump: u8,
}

impl DataLen for SplitVault {
    const LEN: usize = core::mem::size_of::<SplitVault>();
}

pub fn split_vault(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [vault, split, refund] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // loading ix data
    let ix_data = unsafe { load_ix_data::<SplitVault>(data)? };

    // assembling message
    let mut message = [0u8; 72];
    message[0..8].clone_from_slice(&ix_data.amount.to_le_bytes()); // amount to split
    message[8..40].clone_from_slice(split.key()); // split account pubkey
    message[40..].clone_from_slice(refund.key()); // refund account pubkey

    // recovring our pubkey hash from the signature
    let hash = ix_data.signature.recover_pubkey(&message).merklize();

    // PDA equivalence check
    if solana_nostd_sha256::hashv(&[
        hash.as_ref(),
        &[ix_data.bump],
        crate::ID.as_ref(),
        b"ProgramDerivedAddress",
    ])
    .ne(vault.key())
    {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // closing the vault, send split balance to split account, refund remainder to refund account
    *split.try_borrow_mut_lamports()? += ix_data.amount as u64; // Convert u8 to u64
    *refund.try_borrow_mut_lamports()? += vault.lamports().saturating_sub(ix_data.amount as u64); // Convert u8 to u64

    // Close the vault account
    vault.close()?;

    Ok(())
}

use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};
use solana_winternitz::signature::WinternitzSignature;

use crate::states::{load_ix_data, DataLen};

#[repr(C)]
pub struct CloseVault {
    pub signature: WinternitzSignature,
    pub bump: u8,
}

impl DataLen for CloseVault {
    const LEN: usize = core::mem::size_of::<CloseVault>();
}

pub fn close_vault(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [vault, refund] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // load ix data
    let ix_data = unsafe { load_ix_data::<CloseVault>(data)? };

    // recovering pubkey has for sig
    let hash = ix_data.signature.recover_pubkey(refund.key()).merklize();

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

    // closing the vault and refund balance to refund account
    *refund.try_borrow_mut_lamports()? += vault.lamports();
    vault.close()?;

    Ok(())
}

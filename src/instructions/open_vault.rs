use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use pinocchio_system::instructions::CreateAccount;

use crate::states::{load_ix_data, DataLen, VaultState};

#[repr(C)]
pub struct OpenVault {
    pub hash: u8,
    pub bump: u8,
}

impl DataLen for OpenVault {
    const LEN: usize = core::mem::size_of::<OpenVault>();
}

pub fn open_vault(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [payer, vault, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if !payer.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if !vault.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let rent = Rent::get()?;

    let ix_data = unsafe { load_ix_data::<OpenVault>(data)? };

    VaultState::validate_pda(ix_data.bump, vault.key(), payer.key())?;

    // signer seeds
    let hash_bytes = [ix_data.hash];
    let bump_bytes = [ix_data.bump];

    let signer_seeds = [Seed::from(&hash_bytes[..]), Seed::from(&bump_bytes[..])];

    let signers = [Signer::from(&signer_seeds[..])];

    CreateAccount {
        from: payer,
        to: vault,
        space: VaultState::LEN as u64,
        owner: &crate::ID,
        lamports: rent.minimum_balance(VaultState::LEN),
    }
    .invoke_signed(&signers)?;

    unsafe {
        let vault_data = vault.borrow_mut_data_unchecked();
        let vault_state_ref =
            crate::states::utils::load_acc_mut_unchecked::<VaultState>(vault_data)?;
        *vault_state_ref = VaultState {
            owner: *payer.key(),
        };
    }

    Ok(())
}

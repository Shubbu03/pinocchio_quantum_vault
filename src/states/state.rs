use super::utils::DataLen;
use pinocchio::{
    program_error::ProgramError,
    pubkey::{self, Pubkey},
};

use crate::errors::MyProgramError;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VaultState {
    pub owner: Pubkey,
}

impl DataLen for VaultState {
    const LEN: usize = core::mem::size_of::<VaultState>();
}

impl VaultState {
    pub const SEED: &'static str = "quantum_vault";

    pub fn validate_pda(bump: u8, pda: &Pubkey, owner: &Pubkey) -> Result<(), ProgramError> {
        let seed_with_bump = &[Self::SEED.as_bytes(), owner, &[bump]];
        let derived = pubkey::create_program_address(seed_with_bump, &crate::ID)?;
        if derived != *pda {
            return Err(MyProgramError::PdaMismatch.into());
        }
        Ok(())
    }
}

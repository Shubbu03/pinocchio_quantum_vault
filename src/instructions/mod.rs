use pinocchio::program_error::ProgramError;

pub mod close_vault;
pub mod open_vault;
pub mod split_vault;

pub use close_vault::*;
pub use open_vault::*;
pub use split_vault::*;

#[repr(u8)]
pub enum ProgramInstruction {
    OpenVault,
    SplitVault,
    CloseVault,
}

impl TryFrom<&u8> for ProgramInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(ProgramInstruction::OpenVault),
            1 => Ok(ProgramInstruction::SplitVault),
            2 => Ok(ProgramInstruction::CloseVault),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

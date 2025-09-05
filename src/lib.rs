#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

pub mod errors;
pub mod instructions;
pub mod states;

pinocchio_pubkey::declare_id!("HPp4cx5cigaRBzff6YreZtQ8RoqjZJ3CSPFfd4PmUX7P");
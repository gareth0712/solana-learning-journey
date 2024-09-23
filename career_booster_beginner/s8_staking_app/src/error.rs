// We need to provide a concrete error when an instruction is invalid
use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum StakingError {
  /// Invalid instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,
  /// Invalid signer
  #[error("Invalid signer")]
  InvalidSigner,
  /// Invalid owner
  #[error("Invalid owner")]
  InvalidOwner,
  /// Account already initialized
  #[error("Account already initialized")]
  AlreadyInitialized,
  /// Invalid user storage PDA account
  #[error("Invalid user storage PDA")]
  InvalidUserStoragePda,
  /// Invalid SystemProgram account
  #[error("Invalid SystemProgram account")]
  SystemProgramMismatch,
  /// Account is not initialized
  #[error("Account is not initialized")]
  NotInitialized,
}

// impl From<StakingError> for ProgramError is just a convertor from our error to Solana error. That’s it for declaring our errors; we will fill this enum with more once we advance with the logic.
impl From<StakingError> for ProgramError {
  fn from(e: StakingError) -> Self {
    ProgramError::Custom(e as u32)
  }
}

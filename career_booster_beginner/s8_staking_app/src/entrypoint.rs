// src/entrypoint.rs that’s an entry point of our program; this module requires very few code as there is not much logic related to the entry point
use solana_program::{
  account_info::AccountInfo,
  entrypoint,
  entrypoint::ProgramResult,
  pubkey::Pubkey,
};

use crate::processor::process;

entrypoint!(process_instruction);
fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {
  process(program_id, accounts, instruction_data)
}

use borsh::BorshDeserialize;
use s2_hello_pda::{ process_instruction, InstructionData };
use solana_program_test::*;
use solana_sdk::{
  account::Account,
  instruction::{ AccountMeta, Instruction },
  pubkey::Pubkey,
  signature::Signer,
  transaction::Transaction,
};
use std::mem;

#[tokio::test]
async fn test_hello_pda() {
  // some unique pubkey for our program
  let program_id = Pubkey::new_unique();
  // and its storage
  let greeted_pubkey = Pubkey::new_unique();

  let mut program_test = ProgramTest::new(
    "s2_hello_pda", // Run the BPF version with `cargo test-bpf`
    program_id,
    processor!(process_instruction) // Run the native version with `cargo test`
  );

  // TODO: Continue the test implementation here
}

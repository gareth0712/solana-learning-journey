use borsh::BorshDeserialize;
use s1_hello_solana::{ process_instruction, GreetingAccount };
use solana_program_test::*;
use solana_sdk::{
  account::Account,
  instruction::{ AccountMeta, Instruction },
  pubkey::Pubkey,
  signature::Signer,
  transaction::Transaction,
};
use std::mem;

// Integration Tests
#[tokio::test]
async fn test_hello_solana() {
  // some unique pubkey for our program
  let program_id = Pubkey::new_unique();
  // and its storage
  let greeted_pubkey = Pubkey::new_unique();

  let mut program_test = ProgramTest::new(
    "s1_hello_solana", // Run the BPF version with `cargo test-bpf`
    program_id,
    processor!(process_instruction) // Run the native version with `cargo test`
  );

  // create storage account
  program_test.add_account(greeted_pubkey, Account {
    lamports: 5, // should be enough for storing single u32
    data: vec![0_u8; mem::size_of::<u32>()], // mem::size_of::<u32>() returns 4 (as u32 is 4 bytes) => a vector of [0, 0, 0, 0] (initialized value is 0) is created for storing a u32; explicit type annotation `_u8` is necessary here; without `_u8`, it would be a vector of 4 u32s
    owner: program_id, // owner is our program_id
    ..Account::default() // everything else is taken from https://docs.rs/solana-sdk/latest/solana_sdk/account/struct.Account.html
  });

  let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

  let greeted_account = banks_client
    .get_account(greeted_pubkey).await
    .expect("get_account")
    .expect("greeted_account not found");

  assert_eq!(GreetingAccount::try_from_slice(&greeted_account.data).unwrap().counter, 0);

  // Greet once tx
  let mut transaction = Transaction::new_with_payer(
    &[
      // Here is how we create ordinary instruction
      Instruction::new_with_bincode(
        program_id,
        &[0], // ignored but makes the instruction unique in the slot

        vec![AccountMeta::new(greeted_pubkey, false)]
      ),
    ],
    Some(&payer.pubkey())
  );

  // Before Cluster can execute a transaction, it needs to be signed
  transaction.sign(&[&payer], recent_blockhash);
  // 👉 We will send the transaction to the network and the network will complete the transaction successfully or fail in case we did something wrong.
  banks_client.process_transaction(transaction).await.unwrap();

  // Get the account again
  let greeted_account = banks_client
    .get_account(greeted_pubkey).await
    .expect("get_account")
    .expect("greeted_account not found");

  assert_eq!(GreetingAccount::try_from_slice(&greeted_account.data).unwrap().counter, 1)
}

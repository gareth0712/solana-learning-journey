use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
  account_info::AccountInfo,
  entrypoint,
  system_instruction,
  entrypoint::ProgramResult,
  program::invoke_signed,
  account_info::next_account_info,
  pubkey::Pubkey,
  msg,
};

entrypoint!(process_instruction);

// The custom instruction processed by our program. It includes the
// PDA's bump seed, which is derived by the client program. This
// definition is also imported into the off-chain client program.
// The computed address of the PDA will be passed to this program via
// the `accounts` vector of the `Instruction` type.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
  pub vault_bump_seed: u8,
  pub lamports: u64,
}

// The size in bytes of a vault account. The client program needs
// this information to calculate the quantity of lamports necessary
// to pay for the account's rent.
pub static VAULT_ACCOUNT_SIZE: u64 = 1024;
// The entrypoint of the on-chain program, as provided to the
// `entrypoint!` macro.
fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {
  let account_info_iter = &mut accounts.iter(); // Only one mutable iterator is needed

  let payer = next_account_info(account_info_iter)?; // First account: the payer
  // The vault PDA, derived from the payer's address
  // Here we see that we give the vault account which we are going to create, but how do we know its public key ahead if we are supposed to generate it with our program?
  // We can do it because the generation algorithm is deterministic and always produce the same output for the same input. Below is an example of the code that we can use to do so:
  let vault = next_account_info(account_info_iter)?; // Second account: the vault PDA
  let system_program = next_account_info(account_info_iter)?; // Third account: the system program

  let mut instruction_data = instruction_data;
  let instr = InstructionData::deserialize(&mut instruction_data)?;
  let vault_bump_seed = instr.vault_bump_seed;
  let lamports = instr.lamports;
  let vault_size = VAULT_ACCOUNT_SIZE;

  // Invoke the system program to create an account while virtually
  // signing with the vault PDA, which is owned by this caller program.
  invoke_signed(
    &system_instruction::create_account(&payer.key, &vault.key, lamports, vault_size, &program_id),
    &[payer.clone(), vault.clone(), system_program.clone()],
    // A slice of seed slices, each seed slice being the set
    // of seeds used to generate one of the PDAs required by the
    // callee program, the final seed being a single-element slice
    // containing the `u8` bump seed.

    &[&[b"vault", payer.key.as_ref(), &[vault_bump_seed]]]
  )?;
  Ok(())
}

// For the fn called at last, it is a system instruction, i.e. we are interacting with the system program. The function params are as follows
// invoke_signed(
//     instruction: &Instruction,         // The instruction to execute (e.g., create an account)
//     account_infos: &[AccountInfo],     // The accounts involved in the instruction
//     signers_seeds: &[&[u8]]            // The seeds used to sign for PDAs
// ) -> ProgramResult;
// •	invoke is used when normal accounts (with private keys) are involved, and you need the actual account holder to sign the transaction.
// •	invoke_signed is a function that allows a program to “sign” a transaction on behalf of a Program-Derived Address (PDA).
// •	Since PDAs don’t have private keys, the program uses the seeds that were used to derive the PDA to prove it is authorized to act on behalf of the PDA.
// •	It is commonly used when the program needs to create or modify PDAs in Solana, as PDAs cannot sign transactions themselves.

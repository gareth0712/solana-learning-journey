use borsh::{ BorshDeserialize, BorshSerialize, to_vec };
use solana_program::{
  account_info::{ AccountInfo },
  entrypoint,
  entrypoint::ProgramResult,
  program_error::ProgramError,
  account_info::next_account_info,
  msg,
  pubkey::Pubkey,
};

/// Define the type of state stored in accounts/
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
  /// number of greetings
  pub counter: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
  program_id: &Pubkey, // Public key of the account the hello world program was loaded into
  accounts: &[AccountInfo], // The account to say hello to
  _instruction_data: &[u8] // Ignored, all hello world instructions are hellos
) -> ProgramResult {
  msg!("Hello World Rust program entrypoint");

  // Iterating accounts is safer than indexing
  let accounts_iter = &mut accounts.iter();

  // Get the account to say hello to
  let account = next_account_info(accounts_iter)?;
  // ☝️ Unfortunately, there is no verification on the client side of what accounts are expected by the program and in which order. So a list of accounts is hidden knowledge of our program, in our case our program expects that the very first account is a storage account that is owned by our program.

  // Now we check that account is actually owned by our program because otherwise we neither can write there nor trust the data stored on it.
  // The account must be owned by the program in order to modify its data
  if account.owner != program_id {
    msg!("Greeted account does not have the correct program id");
    return Err(ProgramError::IncorrectProgramId);
  }

  // The following is from official example which might not be easily understood:
  // Increment and store the number of times the account has been greeted
  // account.data is RefCell<Vec<u8>> -> This means account.data is a RefCell that contains a Vec<u8>. A RefCell allows for interior mutability, meaning you can borrow data mutably or immutably at runtime.
  // account.data.borrow() -> &Vec<u8> -> Calling borrow() on a RefCell returns an immutable reference to the underlying data, which is a Vec<u8> in this case.
  // &account.data.borrow() -> &[u8] -> Here, you're coercing the reference to Vec<u8> (from borrow()) into a slice (&[u8]).
  // &account.data.borrow()[..] -> &[u8] -> This is explicitly slicing the entire vector and returning a reference to that slice (&[u8]).
  //   let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
  //   greeting_account.counter += 1;
  //   greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

  // Another way of doing the same thing as the above 3 commented lines
  let mut greeting_array = account.data.try_borrow_mut().unwrap();
  let mut greeting_data: GreetingAccount = GreetingAccount::try_from_slice(
    &greeting_array[..]
  ).unwrap();
  greeting_data.counter += 1;
  greeting_array[..].copy_from_slice(&to_vec(&greeting_data).unwrap()[..]);

  msg!("Greeted {} time(s)!", greeting_data.counter);

  Ok(())
}

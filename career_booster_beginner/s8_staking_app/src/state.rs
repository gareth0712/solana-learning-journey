use borsh::{ BorshSerialize, BorshDeserialize };
use solana_program::{ pubkey::Pubkey, program_pack::IsInitialized };

// Let’s plan what we need as a state of our staking pool. We need an authority who owns the staking pool, the amount of staked tokens, the number of users, and how many rewards users earn per staked token. Let’s create the corresponding schema in src/state.rs.
// Define the type of state stored in
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PoolStorageAccount {
  pub pool_authority: Pubkey,

  pub total_staked: u64,
  pub user_count: u64,
  pub rewards_per_token: u64,

  pub is_initialized: bool, // A flag to track if the account is initialized
}

// Implement the `IsInitialized` trait manually
impl IsInitialized for PoolStorageAccount {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}

// Defines schema of user storage account
// Storage size: 8 + 8 + 1 = 16 [reference](https://www.anchor-lang.com/docs/space)
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct UserStorageAccount {
  pub staked: u64,
  pub last_stake_timestamp: UnixTimestamp,

  pub is_initialized: bool,
}
pub const USER_STORAGE_TOTAL_BYTES: usize = 8 + 8 + 1;

impl IsInitialized for UserStorageAccount {
  fn is_initialized(&self) -> bool {
    self.is_initialized
  }
}

//  Remember a good rule of development: Plan-Do-Check-Act, so let’s plan what kind of instructions would need our staking application:

// We definitely need:
// 1. To initialize our staking pool, so we have a place to store information about the pool;
// 2. An instruction to Create users and Stake, so we can store how much concrete user have staked, the timestamp of the last stake, and how much rewards a user has earned;
// 3. allow a user to withdraw his stake with Unstake;
// 4. Claim earned rewards.
use borsh::{ BorshDeserialize, BorshSerialize };

#[derive(BorshDeserialize, BorshSerialize)]
pub enum Instruction {
  Initialize {
    rewards_per_token: u64,
  },
  CreateUser {},
  Stake {
    amount: u64,
  },
  Unstake {
    amount: u64,
  },
  Claim {
    rewards: u64,
  },
}

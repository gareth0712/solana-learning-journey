use anchor_lang::prelude::*;

// This macro defines the address of our program once it is deployed, either localnet, devnet, or mainnet. By default, it is filled with a dummy value that you should update with a real one once you are ready to deploy.
// this address has to be hardcoded within your program because it’s used in owner verification e.g. when accessing program storage account.
declare_id!("6RTLRg3mjopTSDCfEPjEwT2siszbYE7EcafkXw3mT2rS");

// That’s our main entry point; there would be all of the logic we want to implement within our program. That’s how Anchor implements entrypoint.rs, instruction.rs, and processor.rs.
#[program]
pub mod a1_hello_world {
  use super::*;

  pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    Ok(())
  }
}

#[derive(Accounts)]
pub struct Initialize {}

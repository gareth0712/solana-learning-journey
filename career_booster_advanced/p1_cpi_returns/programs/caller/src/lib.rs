use anchor_lang::prelude::*;

declare_id!("8sFaR8wckuARuNRs6zFtMZxfNFwQkjg5PsmfzBep6Bra");

#[program]
pub mod caller {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

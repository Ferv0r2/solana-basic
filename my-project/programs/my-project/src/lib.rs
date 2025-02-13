use anchor_lang::prelude::*;

declare_id!("8JcxXSVWhxmeh3g2rbr7Di2yR7PNeS32unPy5kZRcteh");

#[program]
pub mod my_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

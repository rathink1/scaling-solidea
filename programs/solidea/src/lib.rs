use anchor_lang::prelude::*;

declare_id!("FvfiKfUPLZvrAJnmeAZtMZi51FvYuoWr9WvozxDrb6aE");

#[program]
pub mod solidea {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize {}

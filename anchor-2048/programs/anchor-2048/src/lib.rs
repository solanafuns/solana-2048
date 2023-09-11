use anchor_lang::prelude::*;
mod game;
use game::*;

declare_id!("BRQU3JmG9KwyzgpyTrmvB2BpqonForuoj8tio9SydXrR");

#[program]
pub mod anchor_2048 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.new_game.init();
        Ok(())
    }

    pub fn move_board(ctx: Context<MoveBoard>, direction: u8) -> Result<()> {
        match direction {
            0 => ctx.accounts.game.move_up(),
            1 => ctx.accounts.game.move_down(),
            2 => ctx.accounts.game.move_left(),
            3 => ctx.accounts.game.move_right(),
            _ => unreachable!(),
        }
        ctx.accounts.game.steps += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 512)]
    pub new_game: Account<'info, Game>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MoveBoard<'info> {
    #[account(mut)]
    pub game: Account<'info, Game>,
}

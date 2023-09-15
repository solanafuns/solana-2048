use crate::game::Game;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    declare_id,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

declare_id!("4kUz8auT49b6s1mUiPTdbnmwhPiVLjpaGkVmh81Zjygj");

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum InstructionData {
    GameInit,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
}

pub fn game_init(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let game_account = next_account_info(account_info_iter)?;
    let payer = next_account_info(account_info_iter)?;
    let system_program_account = next_account_info(account_info_iter)?;

    msg!("Debug output: <<<<<<<<<< ");

    for x in accounts {
        msg!("Account ID: {}", x.key);
        msg!("Executable?: {}", x.executable);
        msg!("Is signer: {:#?}", x.is_signer);
        msg!("Writable: {:#?}", x.is_writable);
    }
    msg!("Debug output: >>>>>>>>>>>>> ");

    assert!(payer.is_signer && payer.is_writable && game_account.is_writable);

    const GAME_SPACE: usize = 512;
    let rent = Rent::get()?.minimum_balance(GAME_SPACE);

    invoke(
        &system_instruction::create_account(
            payer.key,
            game_account.key,
            rent,
            GAME_SPACE as u64,
            program_id,
        ),
        &[
            payer.clone(),
            game_account.clone(),
            system_program_account.clone(),
        ],
    )?;

    let mut game_data = Game::new();
    game_data.init();

    msg!("write game data to account {}", game_account.key);
    let mut game_account_mut = game_account.try_borrow_mut_data()?;
    game_data.serialize(&mut *game_account_mut)?;

    Ok(())
}

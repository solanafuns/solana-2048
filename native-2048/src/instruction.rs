use crate::{errors::Error, game::Game};
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

    let mut game_data = Game::new();
    game_data.init();

    // attension here, you should calulate the data size
    let space: usize = game_data.try_to_vec()?.len();
    let rent = Rent::get()?.minimum_balance(space);

    invoke(
        &system_instruction::create_account(
            payer.key,
            game_account.key,
            rent,
            space as u64,
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

    // c.serialize(&mut *game_account_mut)?;

    Ok(())
}

pub fn game_move(accounts: &[AccountInfo], move_instruction: InstructionData) -> ProgramResult {
    for x in accounts {
        msg!("Account ID: {}", x.key);
        msg!("Executable?: {}", x.executable);
        msg!("Is signer: {:#?}", x.is_signer);
        msg!("Writable: {:#?}", x.is_writable);
    }

    let account_info_iter = &mut accounts.iter();
    let _signer_account = next_account_info(account_info_iter)?;
    let game_account = next_account_info(account_info_iter)?;

    let mut game = Game::try_from_slice(&game_account.data.borrow())?;

    assert!(game_account.is_writable);

    match move_instruction {
        InstructionData::MoveUp => game.move_up(),
        InstructionData::MoveDown => game.move_down(),
        InstructionData::MoveLeft => game.move_left(),
        InstructionData::MoveRight => game.move_right(),
        _ => Err(Error::from(Error::UninitializedAccount))?,
    }

    game.serialize(&mut &mut game_account.data.borrow_mut()[..])?;

    Ok(())
}

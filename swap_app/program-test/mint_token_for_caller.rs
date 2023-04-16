use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::{
    instruction::{initialize_account, initialize_mint, mint_to},
    state::{Account, Mint},
};

const MINT_DECIMALS: u8 = 2;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Get the account info of the token account
    let accounts_iter = &mut accounts.iter();
    let token_account_info = next_account_info(accounts_iter)?;

    // Get the account info of the SPL token program
    let spl_token_program_info = next_account_info(accounts_iter)?;

    // Get the account info of the payer
    let payer_info = next_account_info(accounts_iter)?;

    // Get the account info of the mint
    let mint_info = next_account_info(accounts_iter)?;

    // Initialize the mint if it hasn't been already
    let mint_data = Mint::unpack(&mint_info.data.borrow())?;
    if !mint_data.is_initialized {
        initialize_mint(
            spl_token_program_info.key,
            mint_info.clone(),
            payer_info.key,
            None,
            MINT_DECIMALS,
        )?;
    }

    // Mint 10 tokens to the token account
    let token_account_data = Account::unpack(&token_account_info.data.borrow())?;
    if !token_account_data.is_initialized {
        initialize_account(
            spl_token_program_info.key,
            token_account_info.clone(),
            mint_info.key,
            payer_info.key,
        )?;
    }

    mint_to(
        spl_token_program_info.key,
        mint_info.key,
        token_account_info.key,
        payer_info.key,
        &[],
        10 * 10u64.pow(MINT_DECIMALS.into()),
    )?;

    msg!("Minted 10 tokens to account {}", token_account_info.key);

    Ok(())
}

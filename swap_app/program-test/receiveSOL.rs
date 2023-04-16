use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Get the account info of the recipient
    let accounts_iter = &mut accounts.iter();
    let recipient_account = next_account_info(accounts_iter)?;

    // Get the account info of the system program
    let system_program_account = next_account_info(accounts_iter)?;

    // Get the account info of the rent sysvar account
    let rent_sysvar_account = next_account_info(accounts_iter)?;

    // Get the amount of SOL to receive from the instruction data
    let amount = u64::from_le_bytes(instruction_data.try_into().unwrap());

    // Check that the recipient account is not already initialized
    let is_recipient_account_initialized = recipient_account.lamports() != 0;
    if is_recipient_account_initialized {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Get the rent data from the rent sysvar account
    let rent = &solana_program::sysvar::rent::Rent::from_account_info(rent_sysvar_account)?;

    // Calculate the required lamports to rent the recipient account
    let required_lamports = rent.minimum_balance(recipient_account.data_len()) + amount;

    // Check that the sender has enough lamports to rent the recipient account
    let sender_account = next_account_info(accounts_iter)?;
    let sender_balance = sender_account.lamports();
    if sender_balance < required_lamports {
        return Err(ProgramError::InsufficientFunds);
    }

    // Rent the recipient account with the system program
    solana_program::system_instruction::create_account(
        sender_account.key,
        recipient_account.key,
        required_lamports,
        recipient_account.data_len() as u64,
        program_id,
    ).unwrap();
    **recipient_account.lamports.borrow_mut() = amount;

    msg!("Received {} SOL from sender account {} into recipient account {}",
        amount,
        sender_account.key,
        recipient_account.key,
    );

    Ok(())
}

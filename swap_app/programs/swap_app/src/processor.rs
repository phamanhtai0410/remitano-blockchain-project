use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    }
    pubkey::Pubkey,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("This is swap app (Allow to swapp SOL with MOVE token)");

    let account_iter = &mut accounts.iter();

    let account = next_account_info(account_iter)?;

    if account.owner != program_id {
        msg!("This account is not owned by this the program id");
        return Err()
    }

    Ok(())
}

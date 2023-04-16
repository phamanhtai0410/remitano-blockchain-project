use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg,
    program_error::PrintProgramError, pubkey::Pubkey,
};

use crate::{error::SwapError, processor::Processor};
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("EntryPoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        error.print::<SwapError>();
        return Err(error);
    }
    Ok(());
}

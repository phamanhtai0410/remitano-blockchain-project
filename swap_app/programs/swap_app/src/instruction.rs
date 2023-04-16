use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum SwapInstruction {
    Swap(u32),
}

impl SwapInstruction {
    pub fn unpack(imput: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val: Result<[u8; 4], _> = rest[..4].try_info();
                match val {
                    Ok(i) => {
                        return Ok(SwapInstruction::Swap(u32::from_le_bytes(i)))
                    },
                    _ => return Err(ProgramError::InvalidInstructionData)
                }
            }
            _ => Err(ProgramError::InvalidInstructionData);
        }
    }
}

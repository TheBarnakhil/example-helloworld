use solana_program::{program_error::ProgramError};
use std::convert::TryInto;


#[derive(Debug)]
pub enum HelloWorldInstruction {
    Increment,
    Decrement,
    Set(u32)
}


impl HelloWorldInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {

        let (&tag , rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => Ok(Self::Increment),
            1 => Ok(Self::Decrement),
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let val : Result<[u8; 4], _> = rest[..4].try_into();
                match val {
                    Ok(data) => {
                        Ok(Self::Set(u32::from_le_bytes(data)))
                    }
                    Err(_) => Err(ProgramError::InvalidInstructionData)
                }
            }
            _ => Err(ProgramError::InvalidInstructionData)
        }
    }
}
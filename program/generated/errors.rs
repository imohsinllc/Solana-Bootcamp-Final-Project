// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Error, FromPrimitive, Debug, Clone)]
pub enum CoinError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid Signer Permission")]
    InvalidSignerPermission,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,

    #[error("Executable Account Expected")]
    ExecutableAccountExpected,

 
}

impl From<CoinError> for ProgramError {
    fn from(e: CoinError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for CoinError {
    fn type_of() -> &'static str {
        "CoinError"
    }
}

impl PrintProgramError for CoinError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            CoinError::InvalidInstruction => msg!("Error: Invalid instruction"),
            CoinError::InvalidSignerPermission => msg!("Error: The account is_signer value is not the expected one"),
            CoinError::NotExpectedAddress => {
                msg!("Error: Not the expected account address")
            }
            CoinError::WrongAccountOwner => msg!("Error: Wrong account owner"),
            CoinError::InvalidAccountLen => msg!("Error: Invalid account length"),
            CoinError::ExecutableAccountExpected => msg!("Error: Executable account expected"),
 
        }
    }
}
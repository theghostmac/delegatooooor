#![cfg(not(feature = "no-entrypoint"))]

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub mod accounts;
pub mod delegator;
pub mod instructions;
pub mod processor;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &mut [AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::processor::Processor::process(program_id, accounts, instruction_data)
}

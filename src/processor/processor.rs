use crate::instructions::delegate_instruction::DelegatooooorInstruction;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        _instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = DelegatooooorInstruction::unpack(_instruction_data)?;

        match instruction {
            DelegatooooorInstruction::GrantPermission => {}
            DelegatooooorInstruction::RevokePermission => {}
            DelegatooooorInstruction::ExecuteTransaction { amount: _amount } => {}
        }

        Ok(())
    }
}

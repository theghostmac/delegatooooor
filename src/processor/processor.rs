use solana_program::program::invoke;
use solana_program::program_pack::Pack;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use spl_token::instruction as token_instruction;

use crate::delegator::delegate::{DelegatePermissions, Permission};
use crate::instructions::delegate_instruction::DelegatooooorInstruction;

pub struct Processor;

impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        accounts: &mut [AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = DelegatooooorInstruction::unpack(instruction_data)?;

        match instruction {
            DelegatooooorInstruction::GrantPermission { allowance } => {
                msg!("Instruction: GrantPermission");
                Self::grant_permission(accounts, allowance)
            }
            DelegatooooorInstruction::RevokePermission => {
                msg!("Instruction: RevokePermission");
                // Create an iterator with immutable references for revoke_permission
                let accounts_iter = accounts.iter();
                Self::revoke_permission(accounts_iter)
            }
            DelegatooooorInstruction::ExecuteTransaction { amount } => {
                msg!("Instruction: ExecuteTransaction");
                // Create an iterator with immutable references for execute_transaction
                let accounts_iter = accounts.iter();
                Self::execute_transaction(accounts_iter, amount)
            }
        }?;

        Ok(())
    }

    fn grant_permission(accounts: &mut [AccountInfo], allowance: u64) -> ProgramResult {
        if accounts.len() < 4 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        let delegator_account = &accounts[0];
        let delegate_account = &mut accounts[1];
        let token_account = &accounts[2];
        let token_program = &accounts[3];

        if !delegator_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if delegate_account.owner != delegator_account.key {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut delegate_permissions_account_data = delegate_account.try_borrow_mut_data()?;
        let mut delegate_permissions =
            DelegatePermissions::unpack_from_slice(&delegate_permissions_account_data)?;

        if !delegate_permissions.is_initialized {
            delegate_permissions.is_initialized = true;
            delegate_permissions.delegator_address = *delegator_account.key;
            delegate_permissions.permissions = vec![Permission::Spend];
        } else {
            delegate_permissions.permissions.push(Permission::Spend);
        }

        DelegatePermissions::pack_into_slice(
            &delegate_permissions,
            &mut delegate_permissions_account_data,
        );

        let approve_instruction = token_instruction::approve(
            token_program.key,
            token_account.key,
            delegate_account.key,
            delegator_account.key,
            &[&delegator_account.key],
            allowance,
        )?;

        invoke(
            &approve_instruction,
            &[
                token_account.clone(),
                delegate_account.clone(),
                delegator_account.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }

    fn revoke_permission<'a>(
        mut accounts_iter: impl Iterator<Item = &'a AccountInfo<'a>>,
    ) -> ProgramResult {
        // Use next_account_info with an iterator over immutable references
        let delegator_account = next_account_info(&mut accounts_iter)?;
        let delegate_account = next_account_info(&mut accounts_iter)?;

        if !delegator_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if delegate_account.owner != delegator_account.key {
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut delegate_permissions_account_data = delegate_account.try_borrow_mut_data()?;
        let mut delegate_permissions =
            DelegatePermissions::unpack_from_slice(&delegate_permissions_account_data)?;

        if delegate_permissions.is_initialized {
            delegate_permissions
                .permissions
                .retain(|&p| p != Permission::Spend);
            DelegatePermissions::pack_into_slice(
                &delegate_permissions,
                &mut delegate_permissions_account_data,
            );
        }

        Ok(())
    }

    fn execute_transaction<'a>(
        mut accounts_iter: impl Iterator<Item = &'a AccountInfo<'a>>,
        amount: u64,
    ) -> ProgramResult {
        // Use next_account_info with an iterator over immutable references
        let delegate_account = next_account_info(&mut accounts_iter)?;
        let source_token_account = next_account_info(&mut accounts_iter)?;
        let destination_token_account = next_account_info(&mut accounts_iter)?;
        let token_program = next_account_info(&mut accounts_iter)?;

        // Validate that the delegate_account is a signer of the transaction
        if !delegate_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Deserialize the delegate permissions from the delegate account's data
        let delegate_permissions_account_data = delegate_account.try_borrow_data()?;
        let delegate_permissions =
            DelegatePermissions::unpack_from_slice(&delegate_permissions_account_data)?;

        // Check if the delegate has the Spend permission
        if !delegate_permissions
            .permissions
            .contains(&Permission::Spend)
        {
            return Err(ProgramError::InvalidAccountData);
        }

        // Attempting transferring SPL tokens from source to destination.

        // Make sure the token_program account provided is the correct SPL Token program
        if *token_program.key != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        // Construct the SPL Token 'transfer' instruction
        let transfer_instruction = spl_token::instruction::transfer(
            token_program.key,
            source_token_account.key,
            destination_token_account.key,
            delegate_account.key, // Delegate is the authority
            &[&delegate_account.key],
            amount,
        )?;

        // Invoke the transfer instruction
        invoke(
            &transfer_instruction,
            &[
                source_token_account.clone(),
                destination_token_account.clone(),
                delegate_account.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }
}

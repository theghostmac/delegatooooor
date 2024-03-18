use solana_program::{account_info::{AccountInfo, next_account_info}, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};
use solana_program::program::invoke;
use solana_program::program_pack::Pack;
use spl_token::instruction as token_instruction;

use crate::delegator::delegate::DelegatePermissions;
use crate::instructions::delegate_instruction::DelegatooooorInstruction;

pub struct Processor;

impl Processor {
    pub fn process(_program_id: &Pubkey,
                   _accounts: &mut [AccountInfo],
                   _instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = DelegatooooorInstruction::unpack(_instruction_data)?;

        match instruction {
            DelegatooooorInstruction::GrantPermission { allowance } => {
                msg!("Instruction: GrantPermission");
                Self::grant_permission(_accounts, allowance)
            }
            DelegatooooorInstruction::RevokePermission => {
                msg!("Instruction: RevokePermission");
                Self::revoke_permission(_accounts)
            }
            DelegatooooorInstruction::ExecuteTransaction { amount: _amount } => {
                msg!("Instruction: ExecuteTransaction");
                Self::execute_transaction(_accounts, _amount)
            }
        }.expect("Error: Instruction not implemented");

        Ok(())
    }

    fn grant_permission(_accounts: &mut [AccountInfo], allowance: u64) -> ProgramResult {
        let account_info_iter = &mut _accounts.iter();
        let delegator_account = next_account_info(account_info_iter)?; // Delegator's account.
        let delegate_account = next_account_info(account_info_iter)?; // Delegate's account.
        let token_account = next_account_info(account_info_iter)?; // Token account to set allowance for.
        let token_program = next_account_info(account_info_iter)?; // SPL Token program account.


        // Validate accounts.
        let signer_account = next_account_info(_accounts)?; // Delegator signer.
        let delegate_account = next_account_info(_accounts)?; // Delegate account.

        if !signer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Check delegate account ownership.
        if delegate_account.owner != signer_account.key {
            return Err(ProgramError::IncorrectProgramId); // Enforce same program ownership.
        }

        // Load delegate permissions account.
        let mut delegate_permissions = DelegatePermissions::unpack_from_slice(
            &delegate_account.data.borrow()[..],
        )?;

        // Initialize delegate account if not initialized.
        if !delegate_permissions.is_initialized {
            delegate_permissions.is_initialized = true;
            delegate_permissions.delegator_address = *signer_account.key;
            delegate_permissions.permissions = Vec::new();
            delegate_permissions.pack_into_slice(&mut delegate_account.data.borrow_mut()[..]);
        }

        // Grant specific permissions.
        delegate_permissions.permissions.push(DelegatePermissions::Permission::Spend);

        // Update delegate account data.
        delegate_permissions.pack_into_slice(&mut delegate_account.data.borrow_mut()[..]);

        // Call the SPL Token program to approve transferring up to `allowance` tokens
        let approve_instruction = token_instruction::approve(
            token_program.key, // SPL Token program ID
            token_account.key, // Token account the delegate is allowed to spend from
            delegate_account.key, // Delegate's account
            delegator_account.key, // Delegator's account (authority)
            &[&delegator_account.key], // Signers
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

    fn revoke_permission(_accounts: &mut [AccountInfo]) -> ProgramResult {
        // Validate accounts (same as grant_permission)
        let signer_account = next_account_info(_accounts)?;
        let delegate_account = next_account_info(_accounts)?;

        if !signer_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Check delegate account ownership.
        if delegate_account.owner != signer_account.key {
            return Err(ProgramError::IncorrectProgramId); // Enforce same program ownership.
        }

        // Load delegate permissions account.
        let mut delegate_permissions = DelegatePermissions::unpack_from_slice(
            &delegate_account.data.borrow()[..],
        )?;

        // Check permissions (ensure owner has permission to revoke).
        // TODO: implement permission checks here.

        // Revoke specific permissions.
        delegate_permissions.permissions.retain(|p| *p != DelegatePermissions::Permission::Spend); // Revoke Spend permission.

        // Update delegate account data.
        delegate_permissions.pack_into_slice(&mut delegate_account.data.borrow_mut()[..]);

        Ok(())
    }

    fn execute_transaction(account: &mut [AccountInfo], amount: u64) -> ProgramResult {
        // Validate accounts.
        let delegate_account = next_account_info(account)?; // Delegate account.

        if !delegate_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Load delegate permissions account.
        let delegate_permissions = DelegatePermissions::unpack_from_slice(
            &delegate_account.data.borrow()[..],
        )?;

        // Check permissions (ensure delegate has permission to spend)
        if !delegate_permissions.permissions.contains(&DelegatePermissions::Permission::Spend) {
            return Err(ProgramError::InvalidAccountData); // Enforce delegate has spending permission.
        }

        // TODO: Execute transaction.
    }
}

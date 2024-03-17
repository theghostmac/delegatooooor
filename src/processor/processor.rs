use crate::instructions::delegate_instruction::DelegatooooorInstruction;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};


use solana_program::program_pack::Pack;
use crate::accounts::account::Account;
use crate::delegator::delegate::DelegatePermissions;

pub struct Processor;

impl Processor {
    pub fn process(_program_id: &Pubkey,
                   _accounts: &mut [AccountInfo],
                   _instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = DelegatooooorInstruction::unpack(_instruction_data)?;

        match instruction {
            DelegatooooorInstruction::GrantPermission => {
                msg!("Instruction: GrantPermission");
                Self::grant_permission(_accounts)
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

    fn grant_permission(_accounts: &mut [AccountInfo]) -> ProgramResult {
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

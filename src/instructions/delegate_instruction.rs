use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum DelegatooooorInstruction {
    /// Grant permission to a delegate.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the wallet granting permission.
    /// 1. `[writable]` The account of the delegate.
    GrantPermission,

    /// Revoke permission from a delegate.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the wallet revoking permission.
    /// 1. `[writable]` The account of the delegate.
    RevokePermission,

    /// Executes a transaction on behalf of a delegate.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the delegate executing the transaction.
    ExecuteTransaction { amount: u64 },
}

impl DelegatooooorInstruction {
    /// Unpack a byte buffer into a [DelegatooooorInstruction].
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => Self::GrantPermission,
            1 => Self::RevokePermission,
            2 => Self::ExecuteTransaction { amount: u64::from_le_bytes(*array_ref![rest, 0, 8]) },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
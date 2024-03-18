use arrayref::array_ref;
use solana_program::program_error::ProgramError;

#[derive(Debug)]
pub enum DelegatooooorInstruction {
    /// Grant permission to a delegate.
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the wallet granting permission.
    /// 1. `[writable]` The account of the delegate.
    GrantPermission { allowance: u64 },

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
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                // Ensure there are enough bytes for an u64 value for allowance.
                if rest.len() >= 8 {
                    let (allowance_bytes, _) = rest.split_at(8);
                    Self::GrantPermission {
                        allowance: u64::from_le_bytes(*array_ref![allowance_bytes, 0, 8]),
                    }
                } else {
                    return Err(ProgramError::InvalidInstructionData);
                }
            }
            1 => Self::RevokePermission,
            2 => {
                // Ensure there are enough bytes for an u64 value for amount.
                if rest.len() >= 8 {
                    let (amount_bytes, _) = rest.split_at(8);
                    Self::ExecuteTransaction {
                        amount: u64::from_le_bytes(*array_ref![amount_bytes, 0, 8]),
                    }
                } else {
                    return Err(ProgramError::InvalidInstructionData);
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

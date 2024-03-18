use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

pub const DELEGATOR_ADDRESS_SIZE: usize = 32;
pub const IS_INITIALIZED_SIZE: usize = 1;
pub const PERMISSIONS_LENGTH_SIZE: usize = 4;
pub const PERMISSION_SIZE: usize = 1;

pub const DELEGATE_PERMISSIONS_ACCOUNT_SIZE: usize = DELEGATOR_ADDRESS_SIZE
    + IS_INITIALIZED_SIZE
    + PERMISSIONS_LENGTH_SIZE; // This will be the base size without the actual permissions.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Permission {
    Spend = 0,
}

impl From<u8> for Permission {
    fn from(value: u8) -> Self {
        match value {
            0 => Permission::Spend,
            _ => panic!("Unsupported permission!") // TODO: handle this more gracefully.
        }
    }
}

impl Into<u8> for Permission {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Default)]
pub struct DelegatePermissions {
    pub delegator_address: Pubkey,
    pub is_initialized: bool,
    pub permissions: Vec<Permission>,
    pub approved_tokens: Vec<ApprovedToken>
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApprovedToken {
    pub token_account: Pubkey,
    pub allowance: u64,
}

impl ApprovedToken {
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 40];
        let (token_account_dst, allowance_dst) = mut_array_refs![dst, 32, 8];
        *token_account_dst = self.token_account.to_bytes();
        *allowance_dst = self.allowance.to_le_bytes();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 40];
        let (token_account_src, allowance_src) = array_refs![src, 32, 8];
        Ok(ApprovedToken {
            token_account: Pubkey::new_from_array(*token_account_src),
            allowance: u64::from_le_bytes(*allowance_src),
        })
    }
}

impl Sealed for DelegatePermissions {}
impl IsInitialized for DelegatePermissions {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for DelegatePermissions {
    const LEN: usize = DELEGATOR_ADDRESS_SIZE + IS_INITIALIZED_SIZE + PERMISSIONS_LENGTH_SIZE + 4; // Base size, excluding dynamic parts

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let (delegator_address_dst, rest) = dst.split_at_mut(DELEGATOR_ADDRESS_SIZE);
        // Use .copy_from_slice() to copy the bytes from the array to the slice
        delegator_address_dst.copy_from_slice(&self.delegator_address.to_bytes());

        rest[0] = self.is_initialized as u8;

        let permissions_length = self.permissions.len() as u32;
        rest[1..5].copy_from_slice(&permissions_length.to_le_bytes());

        let mut offset = 5; // Start after fixed size parts
        for permission in &self.permissions {
            rest[offset] = (*permission).into();
            offset += 1; // Assuming PERMISSION_SIZE is 1
        }

        let approved_tokens_length = self.approved_tokens.len() as u32;
        rest[offset..offset + 4].copy_from_slice(&approved_tokens_length.to_le_bytes());
        offset += 4;

        for token in &self.approved_tokens {
            token.pack_into_slice(&mut rest[offset..offset + 40]);
            offset += 40;
        }
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let delegator_address = Pubkey::new_from_array(*array_ref![src, 0, 32]);
        let is_initialized = src[32] != 0;

        let permissions_length = u32::from_le_bytes(*array_ref![src, 33, 4]) as usize;
        let mut offset = 37; // Start after fixed size parts

        let mut permissions = Vec::with_capacity(permissions_length);
        for _ in 0..permissions_length {
            permissions.push(Permission::from(src[offset]));
            offset += PERMISSION_SIZE;
        }

        let approved_tokens_length = u32::from_le_bytes(*array_ref![src, offset, 4]) as usize;
        offset += 4;

        let mut approved_tokens = Vec::with_capacity(approved_tokens_length);
        for _ in 0..approved_tokens_length {
            let token = ApprovedToken::unpack_from_slice(&src[offset..offset + 40])?;
            approved_tokens.push(token);
            offset += 40;
        }

        Ok(DelegatePermissions {
            delegator_address,
            is_initialized,
            permissions,
            approved_tokens,
        })
    }
}
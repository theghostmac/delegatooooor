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

#[derive(Debug, Clone, Copy)]
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

#[derive(Clone, Debug, Default, PartialEq)]
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
    const LEN: usize = DELEGATE_PERMISSIONS_ACCOUNT_SIZE;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let (
            delegator_address_dst,
            is_initialized_dst,
            permissions_length_dst,
        approved_tokens_length_dst) =
            mut_array_refs![dst, 32, 1, 4, 4];

        *delegator_address_dst = self.delegator_address.to_bytes();
        is_initialized_dst[0] = self.is_initialized as u8;

        let permissions_length = self.permissions.len() as u32;
        *permissions_length_dst = permissions_length.to_le_bytes();

        // Serialize the permissions, assuming each permission is one byte.
        let mut offset = 32 + 1 + 4; // Sum of sizes of previous parts
        for (i, permission) in self.permissions.iter().enumerate() {
            dst[offset + i] = (*permission).into();
        }
        offset += permissions_length as usize;

        // Now handle approved_tokens
        let approved_tokens_length = self.approved_tokens.len() as u32;
        *approved_tokens_length_dst = approved_tokens_length.to_le_bytes();

        for (i, token) in self.approved_tokens.iter().enumerate() {
            let start = offset + (i * 40); // 40 bytes for each ApprovedToken
            token.pack_into_slice(&mut dst[start..start + 40]);
        }
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let (delegator_address_src, is_initialized_src, permissions_length_src, approved_tokens_length_src) =
            array_refs![src, 32, 1, 4, 4];

        let delegator_address = Pubkey::new_from_array(*delegator_address_src);
        let is_initialized = match is_initialized_src {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let permissions_length = u32::from_le_bytes(*permissions_length_src) as usize;
        let permissions_start_index = DELEGATOR_ADDRESS_SIZE + IS_INITIALIZED_SIZE + PERMISSIONS_LENGTH_SIZE + PERMISSIONS_LENGTH_SIZE; // Adjusted for the additional approved_tokens_length field
        let permissions_end_index = permissions_start_index + permissions_length;

        if src.len() < permissions_end_index {
            return Err(ProgramError::InvalidAccountData);
        }

        let permissions_src = &src[permissions_start_index..permissions_end_index];
        let mut permissions = Vec::with_capacity(permissions_length);
        for permission_byte in permissions_src.iter() {
            permissions.push(Permission::from(*permission_byte));
        }

        // Handling approved_tokens_length and approved_tokens deserialization
        let approved_tokens_length = u32::from_le_bytes(*approved_tokens_length_src) as usize;
        let mut approved_tokens = Vec::with_capacity(approved_tokens_length);
        let mut current_index = permissions_end_index;
        for _ in 0..approved_tokens_length {
            if current_index + 40 > src.len() {
                return Err(ProgramError::InvalidAccountData);
            }
            let token_slice = &src[current_index..current_index + 40];
            let approved_token = ApprovedToken::unpack_from_slice(token_slice)?;
            approved_tokens.push(approved_token);
            current_index += 40; // Move to the next token position
        }

        Ok(DelegatePermissions {
            delegator_address,
            is_initialized,
            permissions,
            approved_tokens,
        })
    }
}

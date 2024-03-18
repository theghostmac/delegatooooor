use arrayref::{array_refs, mut_array_refs};
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
        let (delegator_address_dst, is_initialized_dst, permissions_length_dst) =
            mut_array_refs![dst, 32, 1, 4];

        *delegator_address_dst = self.delegator_address.to_bytes();
        is_initialized_dst[0] = self.is_initialized as u8;

        let permissions_length = self.permissions.len() as u32;
        *permissions_length_dst = permissions_length.to_le_bytes();

        // Start writing permissions after the fixed-size parts.
        let permissions_start = 32 + 1 + 4; // Sum of sizes of previous parts
        let permissions_dst = &mut dst[permissions_start..permissions_start + (permissions_length as usize)];

        // Serialize the permissions, assuming each permission is one byte.
        for (i, permission) in self.permissions.iter().enumerate() {
            permissions_dst[i] = *permission as u8;
        }
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let (delegator_address_src, is_initialized_src, permissions_length_src) =
            array_refs![src, DELEGATOR_ADDRESS_SIZE, IS_INITIALIZED_SIZE, PERMISSIONS_LENGTH_SIZE];

        let delegator_address = Pubkey::new_from_array(*delegator_address_src);
        let is_initialized = match is_initialized_src {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let permissions_len = u32::from_le_bytes(*permissions_length_src) as usize;

        // Calculate the start and end indices of the permissions data in the source slice
        let permissions_start_index = DELEGATOR_ADDRESS_SIZE + IS_INITIALIZED_SIZE + PERMISSIONS_LENGTH_SIZE;
        let permissions_end_index = permissions_start_index + permissions_len;

        // Ensure the slice contains enough data for the permissions
        if src.len() < permissions_end_index {
            return Err(ProgramError::InvalidAccountData);
        }

        let permissions_src = &src[permissions_start_index..permissions_end_index];

        let mut permissions = Vec::with_capacity(permissions_len);
        for permission_byte in permissions_src.iter() {
            permissions.push(Permission::from(*permission_byte));
        }

        Ok(DelegatePermissions {
            delegator_address,
            is_initialized,
            permissions,
        })
    }
}

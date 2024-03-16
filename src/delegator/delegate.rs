use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::program_error::ProgramError;
use solana_program::program_pack::{IsInitialized, Pack, Sealed};
use solana_program::pubkey::Pubkey;

/// DELEGATE_PERMISSIONS_ACCOUNT_SIZE defines the size (37 bytes) of a DelegatePermissions account on Solana.
///
/// This accounts for:
/// 0. delegator_address(32 bytes)
/// 1. is_initialized(1 byte)
/// 2. permissions_len(4 bytes)
pub const DELEGATE_PERMISSIONS_ACCOUNT_SIZE: usize = 32 + 1 + 4; // Public key(32) + bool(1) + Vec length(4) for Permissions

#[derive(Debug)]
pub enum Permission {
    Spend,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DelegatePermissions {
    /// The delegator's address.
    pub delegator_address: Pubkey,
    /// If the delegate permissions are initialized.
    pub is_initialized: bool,
    /// Permissions given to the delegate.
    pub permissions: Vec<Permission>,
}

impl Sealed for DelegatePermissions {}

impl IsInitialized for DelegatePermissions {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for DelegatePermissions {
    /// The size of the data in bytes when serialized.
    const LEN: usize = DELEGATE_PERMISSIONS_ACCOUNT_SIZE;

    /// Serialize the DelegatePermissions struct into a byte array.
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, DELEGATE_PERMISSIONS_ACCOUNT_SIZE];
        let (_delegator_address_dst, _is_initialized_dst, _permissions_dst) =
            mut_array_refs![dst, 32, 1, 4];

        *_delegator_address_dst = self.delegator_address.to_bytes();
        _is_initialized_dst[0] = self.is_initialized as u8;

        // Serialize the permissions length as u32  (4 bytes).
        let permissions_length = self.permissions.len() as u32;
        *_permissions_dst = permissions_length.to_le_bytes();
    }

    /// Deserialize the byte array back into the DelegatePermissions struct.
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, DELEGATE_PERMISSIONS_ACCOUNT_SIZE];
        let (delegator_address_src, is_initialized_src, permissions_src) =
            array_refs![src, 32, 1, 4];

        let delegator_address = Pubkey::new_from_array(*delegator_address_src);

        let is_initialized = match is_initialized_src {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        // Deserialize the permissions length as u32 (4 bytes).
        let permissions_len = u32::from_le_bytes(*permissions_src);
        let mut permissions = Vec::with_capacity(permissions_len as usize);

        // Fill the permissions vector with the given length.
        &src[DELEGATE_PERMISSIONS_ACCOUNT_SIZE..];
        for _ in 0..permissions_len {
            let permission = Permission::Spend;
            permissions.push(permission);
        }

        Ok(DelegatePermissions {
            delegator_address,
            is_initialized,
            permissions,
        })
    }
}

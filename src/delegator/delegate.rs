use solana_program::pubkey::Pubkey;

#[derive(Debug)]
pub enum Permission {
    Spend,
}

#[derive(Debug)]
pub struct DelegatePermissions {
    pub delegate_pubkey: Pubkey,
    pub permissions: Vec<Permission>,
}

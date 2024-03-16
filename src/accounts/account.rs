use solana_program::pubkey::Pubkey;

pub struct Account {
    pub wallet_address: Pubkey,
    pub balance: u64,
}

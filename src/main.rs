#![cfg(not(feature = "no-entrypoint"))]

use solana_program::msg;

mod accounts;
mod delegator;

fn main() {
    msg!("Delegatooooor is up and running!");
}

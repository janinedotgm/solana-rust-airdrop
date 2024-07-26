pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use solana_sdk::{signature::{Keypair, Signer}, pubkey::Pubkey};

    #[test]
    fn keygen(){
        let kp = Keypair::new();
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }
    #[test]
    fn airdrop(){}
    #[test]
    fn transfer_sol(){}
}

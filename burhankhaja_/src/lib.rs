// use solana_sdk::{signature::Keypair, signer::Signer};

// fn main() {
//     let random_keypair = Keypair::new();

//     // println!("Public key : {:?}", random_keypair.pubkey());
//     println!("public key : {:?}", random_keypair.pubkey());
//     println!("secret key : {:?}", random_keypair.secret_bytes());


// }

use solana_sdk::{signature::Keypair, signer::Signer};

fn main() {
    let random_keypair = Keypair::new();
    println!("public key : {:?}", random_keypair.pubkey());
    println!("secret key : {:?}", random_keypair.secret_bytes());

    let wallet = random_keypair.to_base58_string();
    println!("wallet (base58) {:?}", wallet);
}
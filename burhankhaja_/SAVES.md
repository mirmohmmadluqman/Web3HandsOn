# 20 Nov 2025: Solana SDK Basics and...
```
// use solana_sdk::{signature::Keypair, signer::Signer};

// fn main() {
//     let random_keypair = Keypair::new();

//     // println!("Public key : {:?}", random_keypair.pubkey());
//     println!("public key : {:?}", random_keypair.pubkey());
//     println!("secret key : {:?}", random_keypair.secret_bytes());


// }

// use solana_sdk::{signature::Keypair, signer::Signer};

// fn main() {
//     let random_keypair = Keypair::new();
//     println!("public key : {:?}", random_keypair.pubkey());
//     println!("secret key : {:?}", random_keypair.secret_bytes());

//     let wallet = random_keypair.to_base58_string();
//     println!("wallet (base58) {:?}", wallet);
// }


// --------------------------------------------------------------------------------------------------------------------

// use solana_sdk::signer::{keypair::Keypair, Signer}; @ask this whole is was not working why ??

// // #[tokio::main] // @ask Why this tokio::main ?? ?
// /* async */ fn main() { //@ask Why async and what it is ??
//     let rand_keypair = Keypair::new();
//     println!("Public Key: {}", rand_keypair.pubkey());
//     println!("Secret Key: {:?}", rand_keypair.to_bytes());
// }

// --------------------------------------------------------------------------------------------------------------------

use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

// @answer: `#[tokio::main]` is a macro that sets up the Tokio async runtime.
// It allows you to use `async` functions and `.await` within main.
// It wasn't working because the `tokio` dependency was missing in Cargo.toml.
#[tokio::main]
async fn main() {
    // @answer: `async` means this function can be paused and resumed, allowing other tasks to run.
    // It's essential for operations like network requests (e.g., sending transactions to Solana)
    // so they don't block the entire program while waiting for a response.

    let program_address = pubkey!("11111111111111111111111111111111");
    let seeds = [b"helloWorldlkk".as_ref()];
    let (pda, bump) = Pubkey::find_program_address(&seeds, &program_address);
    println!("PDA: {}", pda);
    println!("Bump: {}", bump);
}
```


# 22 Nov 2025: Program
```
use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey;

#[tokio::main]
async fn main() -> Result<()> {
    let connection = RpcClient::new_with_commitment(
        "https://api.mainnet-beta.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let program_id = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    let account_info = connection.get_account(&program_id).await?;
    println!("{:#?}", account_info);

    Ok(())
}
```
## Impl
```
fn main() {
    let gulbab : Person = Person {name: "gulipeer".to_string(), age: 69u8};
    println!("{:?}", Person::new_rand()); // Person
    //// println!("{:?}", gulbab::new_rand()); // # fails because new_rand() doesnt take self reference, 
    // therefore these type of funcs can only be called on Struct Name
    
    ////  println!("{:?}", gulbab.new_rand())  /// ---> will fail too for same reason
    
    println!("{:?}", gulbab.new_rand_self()) 
    
}


#[derive(Debug)]
struct Person {
    name : String,
    age : u8,
}

impl Person {
    fn new_rand() -> Self {
        Person {
            name : "shahabudin".to_string(),
            age : 91u8
        }
    }
    
      fn new_rand_self(&self) -> Self {
        Self {
            name : self.name.clone(), //// if clone() not prsent will fails >>> check data ownership later in rust doc
            age : self.age
        }
    }
}
```
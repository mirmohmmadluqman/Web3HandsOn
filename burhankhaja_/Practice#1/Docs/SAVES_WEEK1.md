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
###     Impl
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

###     LGBTQ IMPL for practice:
```
fn main() {
    let gulbab: Person = Person {
        name: "gulipeer".to_string(),
        age: 69u8,
    };
    println!("{:?}", gulbab);
    println!("{:?}", Person::new_rand());
    println!("{:?}", gulbab.new_rand_self())
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new_rand() -> Self {
        Person {
            name: "shahabudin".to_string(),
            age: 91u8,
        }
    }

    fn new_rand_self(&self) -> Self {
        Self {
            name: self.name.clone(), //// if clone() not prsent will fails >>> check data ownership later in rust doc
            age: self.age,
        }
    }
}

#[derive(Debug)]
enum RGENDER {
    L,
    G,
    B,
    T,
    Q,
    Others,
}

#[derive(Debug)]
struct LGBTQ {
    name: String,
    real_gender: RGENDER,
    age: u8,
}

impl LGBTQ {
    fn real_one() -> Self {
        Self {
            name: "Ibraheem Rashid Khaja".to_string(),
            real_gender: RGENDER::Others,
            age: 16u8,
        }
    }

    // This is a METHOD (not an associated function) because it takes &self
    // &self = a reference to an existing LGBTQ instance
    // This means: "I need to look at an existing person to create a new one"
    fn also_real(&self) -> Self {
        Self {
            // We can access the current instance's data using self.field_name
            // We use .clone() because String owns heap data and we can't move it out of &self
            name: self.name.clone(), // Copies the name from the existing instance
            
            // For enum types like RGENDER, we need to check if they implement Copy
            // If not, we might need to clone or match on them
            // In this case, let's use a different gender to show we're creating something new
            real_gender: RGENDER::L, // We can still use hardcoded values if we want
            
            // u8 implements Copy, so it's automatically copied (no .clone() needed)
            age: self.age, // Copies the age from the existing instance
        }
    }
}

```

###     After Practice:
```
fn main() {
    println!("-----------------------------------------------------------------------------------");

    let gulbab = Person::new("gulipeer".to_string(), 69);
    println!("{:?}", gulbab);

    println!("{:?}", Person::new_rand());
    println!("{:?}", gulbab.new_rand_self());

    println!("-----------------------------------------------------------------------------------");

    let person1 = LGBTQ::real_one();
    let person2 = person1.also_real();

    println!("{:?}", person1);
    println!("{:?}", person2);

    println!("-----------------------------------------------------------------------------------");
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn new_rand() -> Self {
        Self {
            name: "shahabudin".to_string(),
            age: 91,
        }
    }

    fn new_rand_self(&self) -> Self {
        Self {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RGENDER {
    Others, // ← only keep the one you actually use
}

#[derive(Debug)]
struct LGBTQ {
    name: String,
    real_gender: RGENDER,
    age: u8,
}

impl LGBTQ {
    fn real_one() -> Self {
        Self {
            name: "Ibraheem Rashid Khaja".to_string(),
            real_gender: RGENDER::Others,
            age: 16,
        }
    }

    fn also_real(&self) -> Self {
        Self {
            name: self.name.clone(),
            real_gender: self.real_gender,
            age: self.age,
        }
    }
}

```

### Practice:
```
fn main() {
    println!("-----------------------------------------------------------------------------------");

    let gulbab = Person::new("gulipeer".to_string(), 69);
    println!("{:?}", gulbab);

    println!("{:?}", Person::new_rand());
    println!("{:?}", gulbab.new_rand_self());

    println!("-----------------------------------------------------------------------------------");

    let person1 = LGBTQ::real_one();
    let person2 = person1.also_real();

    println!("{:?}", person1);
    println!("{:?}", person2);

    println!("-----------------------------------------------------------------------------------");
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    fn new_rand() -> Self {
        Self {
            name: "shahabudin".to_string(),
            age: 91,
        }
    }

    fn new_rand_self(&self) -> Self {
        Self {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum RGENDER {
    Others, // ← only keep the one you actually use
}

#[derive(Debug)]
struct LGBTQ {
    name: String,
    real_gender: RGENDER,
    age: u8,
}

impl LGBTQ {
    fn real_one() -> Self {
        Self {
            name: "Ibraheem Rashid Khaja".to_string(),
            real_gender: RGENDER::Others,
            age: 16,
        }
    }

    fn also_real(&self) -> Self {
        Self {
            name: self.name.clone(),
            real_gender: self.real_gender,
            age: self.age,
        }
    }
}
```
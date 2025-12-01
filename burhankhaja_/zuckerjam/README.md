```rust
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

fn main() {
    let client = RpcClient::new("https://api.devnet.solana.com".to_string());

    let payer = Keypair::new();
    let program_id = Pubkey::from_str("YOUR_PROGRAM_ID_HERE").unwrap();

    let ix = Instruction {
        program_id,
        accounts: vec![],
        data: vec![], // no instruction data needed
    };

    let recent = client.get_latest_blockhash().unwrap();

    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent,
    );

    client.send_and_confirm_transaction(&tx).unwrap();
}
```

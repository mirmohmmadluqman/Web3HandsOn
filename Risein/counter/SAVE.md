### **A simple overview placed over the project:**

```
my-token-project/
│
├── programs/
│   └── my_token_program/
│       ├── instructions/
│       │   ├── create_mint.rs         ← MINT ACCOUNT
│       │   ├── create_ata.rs          ← ATA
│       │   ├── create_pda_vault.rs    ← PDA
│       │   └── mint_tokens.rs         ← Uses Mint + ATA
│       │
│       └── state/
│           └── vault_account.rs       ← PDA data stored here
│
├── app/
│   └── src/
│       ├── createMint.ts              ← MINT (frontend)
│       ├── createAta.ts               ← ATA
│       ├── createPda.ts               ← PDA
│       └── utils/pdaSeeds.ts          ← PDA seeds
```

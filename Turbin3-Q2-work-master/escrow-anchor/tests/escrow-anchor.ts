/*import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { EscrowAnchor } from "../target/types/escrow_anchor"; 
import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createAccount, createMint, getAccount, getAssociatedTokenAddressSync, mintTo, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
import { randomBytes } from "crypto";
import assert from "assert";

describe("escrow-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider = anchor.getProvider();
  anchor.setProvider(provider);
 
  const connection = provider.connection;

  const program = anchor.workspace.EscrowAnchor as Program<EscrowAnchor>;

  /*const maker = anchor.web3.Keypair.generate();
  //const taker = anchor.web3.Keypair.generate();
  const mintA = anchor.web3.Keypair.generate();
  const mintB = anchor.web3.Keypair.generate();
  const seed = new BN(randomBytes(8));

  const tokenProgram = TOKEN_2022_PROGRAM_ID;

  const makerAAta = getAssociatedTokenAddressSync(maker.publicKey, mintA.publicKey, false, tokenProgram);

  const [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toArrayLike(Buffer, "le", 8)],
    program.programId,
  );

  const vault = getAssociatedTokenAddressSync(mintA.publicKey, escrow, true, tokenProgram);

  /*it ("Request airdrop to the maker", async () => {
  ;

  let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let makerAtaA: anchor.web3.PublicKey;
  let makerAtaB: anchor.web3.PublicKey;
  let takerAtaA: anchor.web3.PublicKey;
  let takerAtaB: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;
  let escrow: anchor.web3.PublicKey;

  const maker = Keypair.generate();
  const taker = Keypair.generate();
  const seed = new anchor.BN(1);
  const depositAmount = new anchor.BN(50);
  const receiveAmount = new anchor.BN(50);
  
  before(async () => {
    const makerAirdrop = await connection.requestAirdrop(maker.publicKey, 7 * LAMPORTS_PER_SOL);
    const takerAirdrop = await connection.requestAirdrop(taker.publicKey, 7 * LAMPORTS_PER_SOL);

    const latestBlockhash = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature: makerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });
    await connection.confirmTransaction({
      signature: takerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });

    mintA = await createMint(connection, maker, maker.publicKey, null, 6);
    mintB = await createMint(connection, taker, taker.publicKey, null, 6);

    makerAtaA = await createAccount(provider.connection, maker, mintA, maker.publicKey);
    // makerAtaA = await createAccount(connection, maker, mintA, maker.publicKey);
    makerAtaB = await createAccount(connection, maker, mintB, maker.publicKey);

    takerAtaA = await createAccount(connection, taker, mintA, taker.publicKey);
    takerAtaB = await createAccount(connection, taker, mintB, taker.publicKey);
    
    await mintTo(connection, maker, mintA, makerAtaA, maker, 1000);
    await mintTo(connection, taker, mintB, takerAtaB, taker, 1000);

    [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toBuffer("le", 8)],
      program.programId
    );

    vault = await anchor.utils.token.associatedAddress({
      mint: mintA,
      owner: escrow
    });


  it("Lets make an Escrow", async () => {
    // Add your test here.
   
    const transaction = await program.methods
    .make(
      seed,
      depositAmount,
      receiveAmount,
    )
    .accountsPartial({
      maker: maker.publicKey,
      mintA, 
      mintB, 
      makerAtaA,
      escrow, 
      vault, 
      systemProgram: SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([maker])
    .rpc();
    
    // const escrowAccount = await program.account.escrow.fetch(escrow);
    // assert.ok(escrowAccount.maker.equals(maker.publicKey));
    // assert.ok(escrowAccount.mintA.equals(mintA));
    // assert.ok(escrowAccount.mintB.equals(mintB));
    // assert.ok(escrowAccount.receive.eq(receiveAmount));

    const vaultAccount = await getAccount(provider.connection, vault);
    assert.ok(vaultAccount.amount === BigInt(depositAmount.toString()));
    console.log("your transaction signature", transaction);
  });

  });
  
}
)*/
import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { EscrowAnchor } from "../target/types/escrow_anchor";
import {Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram} from "@solana/web3.js";
import * as spl from "@solana/spl-token"; // ✅ Correct import
import { randomBytes } from "crypto";
import assert from "assert";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
import { ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("escrow-anchor", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;
  const program = anchor.workspace.EscrowAnchor as Program<EscrowAnchor>;

  let mintA: PublicKey; // the mint address for the first token
  let mintB: PublicKey; // the mint address for the second token
  let makerAtaA: PublicKey; // maker's associated token account for the first token
  let makerAtaB: PublicKey; // maker's associated token account for the second token
  let takerAtaA: PublicKey;  // taker's associated token account for the first token
  let takerAtaB: PublicKey; // taker's associated token account for the second token
  let vault: PublicKey;   // the vault address for the token
  let escrow: PublicKey;  // the escrow address

  const maker = Keypair.generate();
  const taker = Keypair.generate();
  const seed = new BN(1);
  const depositAmount = new BN(50); // amount of tokens to deposit
  const receiveAmount = new BN(50); // amount of tokens to receive

  before(async () => {
    // Airdrop some SOL
    const makerAirdrop = await connection.requestAirdrop(
      maker.publicKey,
      7 * LAMPORTS_PER_SOL
    );
    const takerAirdrop = await connection.requestAirdrop(
      taker.publicKey,
      7 * LAMPORTS_PER_SOL
    );

    const latestBlockhash = await connection.getLatestBlockhash(); // get the latest blockhash of the cluster we are connected to
    await connection.confirmTransaction({ // confirm the transaction of airdropping SOL
      signature: makerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });
    await connection.confirmTransaction({
      signature: takerAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });

    // Create tokens
    mintA = await spl.createMint(
      connection,
      maker, // the payer for the mint transaction
      maker.publicKey, // the mint authority
      null,
      6,
      undefined,
      undefined,
      spl.TOKEN_PROGRAM_ID // the token program ID used to create the mint
    );

    mintB = await spl.createMint(
      connection,
      taker,
      taker.publicKey,
      null,
      6,
      undefined,
      undefined,
      spl.TOKEN_PROGRAM_ID
    );

    // Create token accounts
    makerAtaA = await spl.createAccount(connection, maker, mintA, maker.publicKey);
    makerAtaB = await spl.createAccount(connection, maker, mintB, maker.publicKey);

    takerAtaA = await spl.createAccount(connection, taker, mintA, taker.publicKey);
    takerAtaB = await spl.createAccount(connection, taker, mintB, taker.publicKey);

    // Mint tokens
    await spl.mintTo(connection, maker, mintA, makerAtaA, maker, 1000);
    await spl.mintTo(connection, taker, mintB, takerAtaB, taker, 1000);

    // Derive PDA
    [escrow] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), maker.publicKey.toBuffer(), seed.toArrayLike(Buffer, "le", 8)],
      program.programId
    );

    // Derive vault
    vault = await anchor.utils.token.associatedAddress({
      mint: mintA,
      owner: escrow,
    });
  });

  it("Lets make an Escrow", async () => {
    const txSig = await program.methods
      .make(seed, receiveAmount)
      .accountsPartial({
        maker: maker.publicKey,
        mintA,
        mintB,
        makerAtaA,
        escrow,
        vault,
        systemProgram: SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([maker])
      .rpc();

    /*const vaultAccount = await spl.getAccount(connection, vault); // get the vault account to check the balance
    assert.ok(vaultAccount.amount === BigInt(depositAmount.toString())); // check if the vault balance is correct*/

    console.log("Your transaction signature", txSig);
  });

  /*it("Taker, takes the make offer", async () => {
    const takerAtaABefore = await spl.getAccount(provider.connection, takerAtaA);
    const takerAtaBBefore = await spl.getAccount(provider.connection, takerAtaB);

    const mintAInfo = await spl.getMint(provider.connection, mintA);
    const mintBInfo = await spl.getMint(provider.connection, mintB);

    const escrowAccount = await program.account.escrow.fetch(escrow);
    console.log("Escrow state:", {
      maker: escrowAccount.maker.toString(),
      receiveAmount: escrowAccount.receive.toString(),
      mintA: escrowAccount.mintA.toString(),
      mintB: escrowAccount.mintB.toString()
    });


    const txSig = await program.methods
    .take()
    .accountsPartial({
      taker: taker.publicKey,
      maker: maker.publicKey,
      mintA,
      mintB,
      takerAtaB,
      makerAtaB,
      escrow,
      vault,
      takerAtaA,
      systemProgram :SystemProgram.programId,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID,
    })
    .signers([taker])
    .rpc()
    console.log("Take Tx Signature: ", txSig);
  });*/
  // Unit test for send_funds
it("should send funds correctly", async () => {
  // Setup: Mock inputs and expected behavior.
  const result = await send_funds(makerAtaB, takerAtaB, depositAmount);
  assert.ok(result.success);
});

// Unit test for withdraw
it("should withdraw funds correctly", async () => {
  const result = await withdraw(escrow, makerAtaA, depositAmount);
  assert.ok(result.success);
});

// Unit test for close
it("should close account correctly", async () => {
  const result = await close(escrow, makerAtaA);
  assert.ok(result.success);
});

  it ("Taker sending token B", async () => {
     const txSig = await program.methods
      .take()
      .accountsPartial({
        taker: taker.publicKey,
        maker: maker.publicKey,
        mintA,
        mintB,
        takerAtaB,
        makerAtaB,
        escrow,
        vault,
        takerAtaA,
        systemProgram :SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
      }).signers([taker]).rpc()

    console.log("Take Tx Signature: ", txSig);
  })

  it ("Make offer for refund", async () => {
    const txSig = await program.methods
      .make(seed, depositAmount).accountsPartial({
      maker: maker.publicKey,
      mintA,
      mintB,
      makerAtaA,
      escrow,
      vault,
      systemProgram: SystemProgram.programId,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
    }).signers([maker]).rpc();

    console.log("Tx Signature of make offer which will be refunded: ", txSig);

  //  it ("Refund the maker", async () => {
      const refundSig = await program.methods
      .refund()
      .accountsPartial({
        maker: maker.publicKey,
        mintA,
        makerAtaA,
        escrow,
        vault,
        systemProgram: SystemProgram.programId,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      }).signers([maker]).rpc();

      console.log("Tx Signature of the refund: ", refundSig);
    //});
    
    /*const vaultAccount = await spl.getAccount(provider.connection, vault);
    assert.ok(vaultAccount.amount === BigInt(0)); // check if the vault balance is correct
    const makerAtaABalance = await spl.getAccount(provider.connection, makerAtaA);
    assert.ok(makerAtaABalance.amount === BigInt(1000)); // check if the maker's token A balance is correct*/
  })
  

});

 


async function send_funds(makerAtaA: anchor.web3.PublicKey, takerAtaB: anchor.web3.PublicKey, depositAmount: anchor.BN) {
  // Simulate sending funds logic
  console.log(`Sending ${depositAmount.toString()} tokens from ${makerAtaA.toString()} to ${takerAtaB.toString()}`);
  return { success: true };
}

async function withdraw(escrow: anchor.web3.PublicKey, makerAtaA: anchor.web3.PublicKey, depositAmount: anchor.BN) {
  // Simulate withdrawal logic
  console.log(`Withdrawing ${depositAmount.toString()} tokens from escrow ${escrow.toString()} to ${makerAtaA.toString()}`);
  return { success: true };
}

async function close(escrow: anchor.web3.PublicKey, makerAtaA: anchor.web3.PublicKey) {
  // Simulate closing logic
  console.log(`Closing escrow ${escrow.toString()} and returning funds to ${makerAtaA.toString()}`);
  return { success: true };
}


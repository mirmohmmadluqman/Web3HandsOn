import { Keypair } from "@solana/web3.js";
import * as fs from "fs";

const keypair = Keypair.generate();
const secretKey = Array.from(keypair.secretKey);
fs.writeFileSync("cluster1/wallet.json", JSON.stringify(secretKey));
console.log("Generated new wallet with valid keypair!");
console.log("Public Key:", keypair.publicKey.toBase58());

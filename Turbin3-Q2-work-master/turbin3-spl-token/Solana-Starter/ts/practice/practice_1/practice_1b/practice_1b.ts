import { Connection, PublicKey } from '@solana/web3.js';

const connection = new Connection("https://api.devnet.solana.com");
// Using a known Solana wallet address for demonstration
const address = new PublicKey("CmBYrHfHssUEN7wckU5E65w4UsK9RNJcmfooTi3orDXX");

// Old way
connection.getBalance(address)
    // ↓ .getBalance() returns a Promise<number>
    .then(balance => {
        // ↑
        // This 'balance' is GIVEN to you by .then()
        // It's the number that getBalance() resolved with
        console.log(`Balance: ${balance}`);
    })
    .catch(error => {
        // ↑
        // This 'error' is GIVEN to you by .catch()
        // It's whatever error was thrown
        console.error(error);
    });
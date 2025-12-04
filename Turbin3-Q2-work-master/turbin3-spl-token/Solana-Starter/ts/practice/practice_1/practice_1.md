## 🎨 Visual Representation
```typescript
fetch('url')
    // ↓ Returns Promise<Response>
    .then(response => response.json())
          ↑                    ↓
          |                    Returns Promise<JSON>
          |
          This value is GIVEN to you by .then()
          You can name it ANYTHING:
          - response
          - res
          - r
          - banana (it would still work!)
    
    .then(data => console.log(data))
          ↑
          |
          This value is GIVEN to you by .then()
          You can name it ANYTHING:
          - data
          - json
          - result
          - pizza (it would still work!)       
```
---
### 💡 Proof: You Can Name It Anything!
```ts
// ✅ This works
fetch('url')
    .then(response => response.json())
    .then(data => console.log(data));

// ✅ This ALSO works (same thing!)
fetch('url')
    .then(banana => banana.json())
    .then(pizza => console.log(pizza));

// ✅ This ALSO works
fetch('url')
    .then(x => x.json())
    .then(y => console.log(y));
```
**The names don't matter!** `.then()` passes the value to whatever parameter name you choose.

## 🔍 How .then() Works Internally
Here's a simplified version of what `.then()` does:
```ts
typescript
class Promise {
    then(callback) {
        // Wait for the promise to resolve
        // When it resolves, call your callback with the result
        const result = /* ... wait for promise ... */;
        return callback(result);  // ← Passes the value to YOUR function
    }
}
```
---
## 📚 Complete Example with Comments
```ts
// Step 1: fetch() returns a Promise that will resolve with a Response object
fetch('https://api.example.com/data')

    // Step 2: When the Promise resolves, .then() receives the Response
    //         and passes it to your function as 'response'
    .then(response => {
        console.log('Got response:', response);
        // response.json() returns a NEW Promise that will resolve with JSON
        return response.json();
    })

    // Step 3: When response.json() resolves, .then() receives the JSON
    //         and passes it to your function as 'data'
    .then(data => {
        console.log('Got data:', data);
    })

    // Step 4: If ANY promise rejects, .catch() receives the error
    //         and passes it to your function as 'error'
    .catch(error => {
        console.error('Got error:', error);
    });
```
---
## 🎯 Solana Example
```ts
import { Connection, PublicKey } from '@solana/web3.js';

const connection = new Connection("https://api.devnet.solana.com");
const address = new PublicKey("YourAddressHere");

// Old way
connection.getBalance(address)
    // ↓ .getBalance() returns a Promise<number>
    .then(balance => {
          ↑
          // This 'balance' is GIVEN to you by .then()
          // It's the number that getBalance() resolved with
        console.log(`Balance: ${balance}`);
    })
    .catch(error => {
          ↑
          // This 'error' is GIVEN to you by .catch()
          // It's whatever error was thrown
        console.error(error);
    });
```
---
## 🧪 Interactive Example You Can Run
Create a file `promise-demo.ts`:
```ts
// Example 1: Simple Promise
const myPromise = new Promise((resolve, reject) => {
    setTimeout(() => {
        resolve("Hello from the Promise!");  // ← This value goes to .then()
    }, 1000);
});

myPromise.then(message => {
    //            ↑
    //            This receives "Hello from the Promise!"
    console.log(message);
});

// Example 2: Chaining
const promise2 = new Promise((resolve) => {
    resolve(5);  // ← This goes to first .then()
});

promise2
    .then(number => {
        //   ↑ Receives 5
        console.log('First then:', number);
        return number * 2;  // ← This goes to second .then()
    })
    .then(doubled => {
        //   ↑ Receives 10
        console.log('Second then:', doubled);
        return doubled + 3;  // ← This goes to third .then()
    })
    .then(final => {
        //   ↑ Receives 13
        console.log('Final then:', final);
    });
```
---
## 📝 Summary

| Code                       | Who provides the value? | You define the name? |
| :------------------------- | :---------------------- | :------------------- |
| `response => response.json()` | `.then()` provides it   | ✅ Yes (you chose "response") |
| `data => console.log(data)` | `.then()` provides it   | ✅ Yes (you chose "data")    |
| `error => console.error(error)` | `.catch()` provides it  | ✅ Yes (you chose "error")   |

#### The pattern:

- A function returns a Promise
- .then() waits for that Promise to resolve
- .then() automatically passes the resolved value to your callback function
- You choose what to name that parameter

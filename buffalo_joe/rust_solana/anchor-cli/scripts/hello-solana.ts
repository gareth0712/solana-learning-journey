import {
  Connection,
  Keypair
} from '@solana/web3.js';

import { connectDevnet, getAccount, getProgram, pingProgram } from '@/utils';

async function main() {
  const programName = 'hello_solana';
  // run the following command in cli ahead to ensure you can get the log message properly: 
  // $ solana logs | grep "<program id deployed on devnet> invoke" -A 3
  console.log("Launching client...");

  const connection: Connection = await connectDevnet();
  // Get our program's public key (ensure you have deployed the program on devnet)
  const { programId } = await getProgram(programName);
  const localKeypair: Keypair = await getAccount();
  await pingProgram({
    connection,
    programName,
    programId,
    localAccountKeypair: localKeypair,
    accountPubkey: localKeypair.publicKey,
  });
  
  /*
    If you have run the following command in cli ahead, you shd see the logs upon finishing executing this program, for my case: 
    $ solana logs | grep "<program id deployed on devnet> invoke" -A 3
    Program 4FwmqYqcwDEZ52HBnoJmqEftxyBjcQRGwLn7E8ZCq6LR invoke [1]
    Program log: Hello Solana! (From Rust!)
    Program 4FwmqYqcwDEZ52HBnoJmqEftxyBjcQRGwLn7E8ZCq6LR consumed 340 of 200000 compute units
    Program 4FwmqYqcwDEZ52HBnoJmqEftxyBjcQRGwLn7E8ZCq6LR success
  */
}


main().then(
  () => process.exit(),
  err => {
    console.log('Error occurred running the cli program')
    console.error(err);
    process.exit(-1);
  },
);
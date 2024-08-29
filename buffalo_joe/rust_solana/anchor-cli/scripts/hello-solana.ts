import {
  Connection,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';

import 'dotenv/config';

import { connectDevnet, getProgram, getAccount } from '@/utils';

async function main() {
  // run the following command in cli ahead to ensure you can get the log message properly: 
  // $ solana logs | grep "<program id deployed on devnet> invoke" -A 3
  console.log("Launching client...");

  const connection: Connection = await connectDevnet();
  // Get our program's public key (ensure you have deployed the program on devnet)
  const { programId } = await getProgram('hello_solana');
  const triggerKeypair = await getAccount();

  /*
  Conduct a transaction with our program
  */
  console.log('--Pinging Program ', programId.toBase58());
  const instruction = new TransactionInstruction({
    keys: [{pubkey: triggerKeypair.publicKey, isSigner: false, isWritable: true}],
    programId,
    data: Buffer.alloc(0),
  });
  await sendAndConfirmTransaction(
    connection,
    new Transaction().add(instruction),
    [triggerKeypair],
  );

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
import {
  Keypair,
  Connection,
  PublicKey,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import fs from 'mz/fs';

import 'dotenv/config';

import { generateKeypair, getProgramKeypairPath, loadKeypairFromFile } from '@/utils';

async function main() {
  // run the following command in cli ahead to ensure you can get the log message properly: 
  // $ solana logs | grep "<program id deployed on devnet> invoke" -A 3
  console.log("Launching client...");

  /*
  Connect to Solana DEV net
  */
  let connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  /*
  Get our program's public key (ensure you have deployed the program on devnet)
  */
  const programKeypairPath: string = getProgramKeypairPath('hello_solana-keypair.json');
  const secretKeyString = await fs.readFile(programKeypairPath, {encoding: 'utf8'});
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  const programKeypair: Keypair = Keypair.fromSecretKey(secretKey);
  let programId: PublicKey = programKeypair.publicKey;
  console.log('Program ID is: ', programId)

  /*
    Keypair of our solana account (ensure there is +ve SOL balance)
  */
  let triggerKeypair;
  const keypairPath: string | undefined = process.env.KEYPAIR_ID_PATH;
  if (keypairPath) {
    triggerKeypair = await loadKeypairFromFile(keypairPath);
  } else {
    triggerKeypair = await generateKeypair(connection);
  }
  console.log('Account public key is: ', triggerKeypair.publicKey);

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
  if you have run the following command in cli ahead, you shd see the logs upon finishing executing this program, for my case: 
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
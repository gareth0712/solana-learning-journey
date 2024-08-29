import {
  Keypair,
  Connection,
  PublicKey,
  LAMPORTS_PER_SOL,
  TransactionInstruction,
  Transaction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';

import 'dotenv/config';

/*
  Keypair we used to create the on-chain Rust program
*/
const PROGRAM_KEYPAIR_PATH = path.join(
  path.resolve(__dirname, '../dist/program'),
  'hello_solana-keypair.json'
);

const loadKeypairFromFile = async (filePath: string): Promise<Keypair> => {
  const absolutePath = path.resolve(filePath);
  const keypairData = JSON.parse(fs.readFileSync(absolutePath, 'utf-8'));
  return Keypair.fromSecretKey(Uint8Array.from(keypairData));
}

const generateKeypair = async (connection): Promise<Keypair> => {
  /*
  Generate an account (keypair) to transact with our program
  */
  const triggerKeypair = Keypair.generate();

  const airdropRequest = await connection.requestAirdrop(
    triggerKeypair.publicKey,
    LAMPORTS_PER_SOL,
  );
  await connection.confirmTransaction(airdropRequest);

  return triggerKeypair;
}

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
  const secretKeyString = await fs.readFile(PROGRAM_KEYPAIR_PATH, {encoding: 'utf8'});
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  const programKeypair = Keypair.fromSecretKey(secretKey);
  let programId: PublicKey = programKeypair.publicKey;
  console.log('Program ID is: ', programId)

  /*
    Keypair of our solana account (ensure there is +ve SOL balance)
  */
  const triggerKeypair = await loadKeypairFromFile(process.env.KEYPAIR_ID_PATH);
  // Use this function if you don't have an existing keypair
  // const triggerKeypair = await generateKeypair(connection);
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
    console.log('Error occurred runnin the cli program')
    console.error(err);
    process.exit(-1);
  },
);
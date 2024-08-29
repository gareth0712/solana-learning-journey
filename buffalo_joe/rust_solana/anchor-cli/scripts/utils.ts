import {
  Keypair,
  Connection,
  LAMPORTS_PER_SOL,
} from '@solana/web3.js';
import path from 'path';
import fs from 'mz/fs';

export const generateKeypair = async (connection: Connection): Promise<Keypair> => {
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

/*
  Keypair we used to create the on-chain Rust program
*/
export const getProgramKeypairPath = (jsonFile: string): string => {
  return path.join(
    path.resolve(__dirname, '../dist/program'),
    'hello_solana-keypair.json'
  );
}

export const loadKeypairFromFile = async (filePath: string): Promise<Keypair> => {
  const absolutePath = path.resolve(filePath);
  const keypairData = JSON.parse(fs.readFileSync(absolutePath, 'utf-8'));
  return Keypair.fromSecretKey(Uint8Array.from(keypairData));
}
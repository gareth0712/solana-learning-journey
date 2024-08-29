import {
  Keypair,
  Connection,
} from '@solana/web3.js';
import fs from 'mz/fs';

/*
  Connect to Solana dev net.
*/
export const connectDevnet = async (): Promise<Connection> => {
  const connection = new Connection('https://api.devnet.solana.com', 'confirmed');

  console.log(`Successfully connected to Solana dev net.`);
  return connection;
}

/*
  Given filePath, load the keypair for further handling
*/
export const loadKeypairFromFile = async (filePath: string): Promise<Keypair> => {
  console.log('Reading file path: ', filePath);
  const secretKeyString = await fs.readFile(filePath, {encoding: 'utf8'});
  const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
  return Keypair.fromSecretKey(Uint8Array.from(secretKey));
}
import {
  Keypair,
  Connection,
} from '@solana/web3.js';
import fs from 'mz/fs';

/*
  Connect to Solana network given rpc url.
*/
export const connectSolRpc = async (rpcUrl: string): Promise<Connection> => {
  console.log(`Connecting to sol rpc: ${rpcUrl}...`)
  const connection = new Connection(rpcUrl, 'confirmed');

  console.log(`Successfully connected to Solana network.`);
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
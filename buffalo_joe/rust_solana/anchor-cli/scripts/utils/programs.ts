import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import path from 'path';

import { PROGRAM_PATH, loadKeypairFromFile } from '.';

/*
  Get the following for targeted program:
  - program keypair: Keypair we used to create the on-chain Rust program
  - programId
*/
export const getProgram = async (programName: string): Promise<{programKeypair: Keypair, programId: PublicKey}> => {
  // e.g. hello_solana => hello_solana-keypair.json
  const programKeypair: Keypair = await loadKeypairFromFile(
      path.join(PROGRAM_PATH, programName + '-keypair.json')
  );
  const programId: PublicKey = programKeypair.publicKey;

  console.log(`We're going to ping the ${programName} program.`);
  console.log(`Its Program ID is:`);
  console.log(`   ${programId.toBase58()}`)
  return { programKeypair, programId};
}

/*
  Ping the program.
*/
export async function pingProgram({
  connection,
  programName,
  programId,
  localAccountKeypair,
  accountPubkey, // either local account or client account
}: {
  connection: Connection,
  programName: string,
  programId: PublicKey,
  localAccountKeypair: Keypair,
  accountPubkey: PublicKey,
}) {
  console.log(`All right, let's run it.`);
  console.log(`Pinging ${programName} program of programId ${programId.toBase58()}...`);
  
  // const instruction = new TransactionInstruction({
  //   keys: [{pubkey: triggerKeypair.publicKey, isSigner: false, isWritable: true}],
  //   programId,
  //   data: Buffer.alloc(0),
  // });
  // await sendAndConfirmTransaction(
  //   connection,
  //   new Transaction().add(instruction),
  //   [triggerKeypair],
  // );

  const instruction = new TransactionInstruction({
      keys: [{pubkey: accountPubkey, isSigner: false, isWritable: true}],
      programId,
      data: Buffer.alloc(0), // Empty instruction data
  });
  await sendAndConfirmTransaction(
      connection,
      new Transaction().add(instruction),
      [localAccountKeypair],
  );

  console.log(`Ping successful.`);
}
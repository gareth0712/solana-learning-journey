import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from '@solana/web3.js';
import path from 'path';

import { PROGRAM_PATH, loadKeypairFromFile, connectSolRpc, getAccount, configureClientAccount, logger } from '.';

/*
  Get the following for targeted program (ensure you have already deployed the program):
  - program keypair: Keypair we used to create the on-chain Rust program
  - programId
*/
export const getProgram = async (programName: string): Promise<{programKeypair: Keypair, programId: PublicKey}> => {
  logger.section(`============= Program ====================`);
  // e.g. hello_solana => hello_solana-keypair.json
  const programKeypair: Keypair = await loadKeypairFromFile(
      path.join(PROGRAM_PATH, programName + '-keypair.json')
  );
  const programId: PublicKey = programKeypair.publicKey;

  logger.log(`We're going to ping the ${programName} program.`);
  logger.success(`Its Program ID is: ${programId.toBase58()}`);
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
  clientAccountPubkey, // supply local account if no state is needed in the program
}: {
  connection: Connection,
  programName: string,
  programId: PublicKey,
  localAccountKeypair: Keypair,
  clientAccountPubkey: PublicKey,
}) {
  logger.section(`============= Pinging Program ============`);
  logger.log(`Pinging ${programName} program of programId ${programId.toBase58()}...`);

  const instruction = new TransactionInstruction({
      keys: [{pubkey: clientAccountPubkey, isSigner: false, isWritable: true}],
      programId,
      data: Buffer.alloc(0), // Empty instruction data
  });
  await sendAndConfirmTransaction(
      connection,
      new Transaction().add(instruction),
      [localAccountKeypair],
  );

  logger.success(`Ping successful.`);
}

export const pingProgramFromConnection = async (programName: string, options?: {
  accountSpaceSize?: number,
  rpcUrl?: string
}) => {
  logger.section(`============ Launching Client ============`);
  const connection: Connection = await connectSolRpc(options?.rpcUrl);
  const { programId } = await getProgram(programName);

  let localAccountKeypair: Keypair, clientPublicKey: PublicKey;
  localAccountKeypair = await getAccount();

  if (options?.accountSpaceSize !== undefined) {
    clientPublicKey = await configureClientAccount({
      connection,
      localAccountKeypair,
      programId,
      accountSpaceSize: options.accountSpaceSize,
    })
  } else {
    clientPublicKey = localAccountKeypair.publicKey;
  }
  await pingProgram({
    connection,
    programName,
    programId,
    localAccountKeypair: localAccountKeypair,
    clientAccountPubkey: clientPublicKey,
  });
}
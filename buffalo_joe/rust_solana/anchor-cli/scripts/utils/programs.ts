import {
  Keypair,
  PublicKey,
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

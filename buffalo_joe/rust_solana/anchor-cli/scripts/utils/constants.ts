import path from 'path';
import os from 'os';

export const PROGRAM_PATH = path.resolve(__dirname, '../../dist/program');

/*
  Path to Solana CLI config file.
*/
export const CONFIG_FILE_PATH = path.resolve(
  os.homedir(),
  '.config',
  'solana',
  'cli',
  'config.yml',
);
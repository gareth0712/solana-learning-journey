fn create_vault_account(client: &RpcClient, program_id: Pubkey, payer: &Keypair) -> Result<()> {
  // Derive the PDA from the payer account, a string representing the unique
  // purpose of the account ("vault"), and the address of our on-chain program.
  // In running our program id and seeds through a hash function, there is a ~50% chance that we end up with a valid public key that does lie on the elliptic curve. In this case, we simply add something to fudge our input and try again. The technical term for this fudge factor is a bump. In Solana, we start with bump = 255 and simply iterate down through bump = 254, bump = 253, etc., until we get an address that is not on the elliptic curve. This may seem rudimentary, but once found, it gives us a deterministic way of deriving the same PDA repeatedly.
  // So (vault_pubkey, vault_bump_seed), result of Pubkey::find_program_address is actually address and bump required kick this address out of ed25519 elliptic curve.
  let (vault_pubkey, vault_bump_seed) = Pubkey::find_program_address(
    &[b"vault", payer.pubkey().as_ref()],
    &program_id
  );
  // Get the amount of lamports needed to pay for the vault's rent
  let vault_account_size = usize::try_from(VAULT_ACCOUNT_SIZE)?;
  let lamports = client.get_minimum_balance_for_rent_exemption(vault_account_size)?;
  // The on-chain program's instruction data, imported from that program's crate.
  let instr_data = InstructionData {
    vault_bump_seed,
    lamports,
  };
  // The accounts required by both our on-chain program and the system program's
  // `create_account` instruction, including the vault's address.
  let accounts = vec![
    AccountMeta::new(payer.pubkey(), true),
    AccountMeta::new(vault_pubkey, false),
    AccountMeta::new(system_program::ID, false)
  ];

  // Create the instruction by serializing our instruction data via borsh
  let instruction = Instruction::new_with_borsh(program_id, &instr_data, accounts);
  let blockhash = client.get_latest_blockhash()?;
  let transaction = Transaction::new_signed_with_payer(
    &[instruction],
    Some(&payer.pubkey()),
    &[payer],
    blockhash
  );
  client.send_and_confirm_transaction(&transaction)?;
  Ok(())
}

fn main() {
  create_vault_account(&client, program_id, &payer).unwrap();
}

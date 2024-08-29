# Build on Solana

In this bootcamp, we'll dive into the essentials of Solana Smart Contracts, covering everything from transactions and instructions to accounts and the Program Derived Addresses (PDAs). 

## Introduction

We will go through four main concepts:

A. Transactions  
B. Instructions  
C. Accounts  
D. Program Derived Addresses (PDAs)

## Transactions

In Solana, a transaction is a signed instruction sequence that is executed atomically. Transactions are used to interact with programs (smart contracts) deployed on the Solana network.

A transaction contains one or more instructions, a recent blockhash, and signatures.

1. The recent blockhash, also known as a transaction "recentBlockhash", is used for transaction processing and lifespan. 
2. The signatures are the cryptographic proof of the transaction's integrity and authority. Each transaction must be signed by the appropriate account holders as per the instructions contained within. 

## Instructions

Instructions

Instructions in Solana are bundled within a transaction and are executed sequentially.

Each instruction contains a program ID, accounts that it wishes to interact with, and a data field.

1. The program ID specifies the on-chain program (smart contract) that will process the instruction. 
2. The accounts array includes all accounts that the instruction will read from or write to. The accounts array must include at least one signer who authorizes the instruction.
3. The data field is an arbitrary byte array that is passed to the program for processing. 

For example, let’s say you want to transfer tokens from one account to another on Solana. This operation would be executed via an instruction.

1. Program ID: The Solana Token Program ID, which is responsible for handling token transfers.
2. Accounts Array:
- The source account (where the tokens are currently held).
- The destination account (where the tokens will be sent).
- The owner or signer’s account (authorizing the transaction).
3. Data Field: The specific amount of tokens to transfer, encoded as a byte array.

In Solana, the combination of transactions and instructions allows for high throughput and efficient processing.

The network is capable of processing thousands of transactions per second, each containing multiple instructions, which can interact with different programs and accounts.

This design is a key part of Solana's scalability and speed.

## Accounts

In Solana, an account is a persistent memory structure that a program can use for storing state. Every account in Solana is initially owned by the system program, but the system program can change the ownership if the correct private key for the account is provided. Each account includes several properties:

1. Public Key (Pubkey): This is the unique identifier of the account.
2. Signer Flag (is_signer): This flag indicates whether the account is a signer of the transaction. If true, the transaction must include the signature of this account.
3. Writable Flag (is_writable): This flag indicates whether the data in the account can be modified. If true, the account's data can be written to in the current transaction.
4. Lamports: This is the number of lamport's (the smallest unit of the native SOL token) held in the account. Lamports also serve as rent to keep the account on the network. 
5. Data: This is a byte array that can hold arbitrary data. 
6. Owner: This is another public key that identifies the program that has authority over the account's data. Only the owner program can modify the account's data.
7. Executable Flag: Hold if the account is a smart contract. 
8. Rent Epoch: This value represents the latest epoch that rent has been paid for the account. Rent is paid in SOL and ensures that the account remains active on the network.

In Solana, accounts not only hold the state of a program but can also be used to create complex relationships between different programs. 

## Program Derived Addresses (PDAs)

A Program Derived Address (PDA) is a special type of account in Solana that is derived from a base public key and a program ID, but does not have a known private key. Despite not having a known private key, a PDA can own data and tokens, and it can sign transactions, but only through the program that it's associated with.

PDAs are useful for a variety of reasons:

1. Security: Since PDAs do not have a known private key, only the program that the PDA is derived from can make changes to the PDA, which provides a strong layer of security.
2. Authority Delegation: PDAs can be used to delegate authority. For example, a program can use a PDA as an intermediary to hold funds. The program can then allow specific users to initiate transactions that transfer funds from the PDA, effectively delegating authority to those users.
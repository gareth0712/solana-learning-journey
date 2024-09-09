# Career Booster Advanced Course

This README combines everything I learnt from the Career Booster beginner and advanced course for
both Solana Blockchain and its program development practice.

# Solana Blockchain

😎 Solana is an energy-efficient Tier 1 blockchain with high transaction processing speeds and the
ability to scale stably. It launched in 2017 with the then-innovative Proof-of-History (PoH)
consensus mechanism, which provides one of the highest transaction processing speeds per second
(more than 700,000 transactions per second ✌). The following year, the development team released a
testnet, and they released the mainnet in 2020.

## Ethereum vs Solana

Ethereum is currently the most prominent blockchain. It hosts more decentralized applications than
any other blockchain ecosystem. However, due to Ethereum's low bandwidth processing capacity of
about 15 transactions per second, the vast enthusiasm around the project makes gas prices
exorbitant.

👉 In contrast, the Solana blockchain combines several new protocols, solving the trilemma of
scalability, security, and decentralization. It takes 0.4 seconds to process a block, and as
hardware capacity increases, so does the speed of the entire network.

This difference in speed is due to the difference in the consensus algorithm.

## Solana Consensus Algorithm

PoW consensus blockchains do a lot of energy-consuming work. Due to the unstable climate situation,
many consider the PoW consensus unsuitable. Low-energy efficiency was one of the main reasons
Ethereum switched from Proof-of-Work consensus to Proof-of-Stake (PoS).

👉 PoS replaces miners with validators, who bid on matching tokens and thus participate in verifying
blocks in the network. They do not need to compete, as miners do, to be the first to solve the
puzzle. Instead, users are randomly selected based on their stakes. The higher the rate, the better
the chance of being chosen by the validator. When a validator is appointed, they must propose a
block. If other users verify that block, the validator receives a reward consisting of the
transaction fee for that block. PoS is slightly less secure than PoW because people, not
mathematical decisions, determine its security. There is also the possibility that with PoS, a group
of validators will take control.

Still, in reality, these validators can be blocked or excluded from the network, so the more
significant the blockchain, the less likely this is. Also, due to the lack of computation, PoS is
less energy intensive.

☝ Solana combines Proof-of-Stake in its Proof-History (PoH) consensus to provide a unique consensus
algorithm. Each transaction or data fragment is assigned a unique timestamp, representing state and
data with cryptographically secure hashes. Timestamps guarantee the order of events and precisely
determine the time of data creation. Although PoH is critical to the consensus model, it is not the
primary consensus protocol. Instead, Solana uses a practical Byzantine fault tolerance mechanism
(pBFT) called Tower Consensus, intertwined with the PoS mechanism. We do not need to dive into it
now; this information is enough to get acquainted with it 😉

The combined efforts of PoS validators using the PoH protocol and optimizing consensus make Solana
one of the world's fastest, safest, and most decentralized blockchains. The Tower Consensus protocol
reduces latency using the PoH protocol as a global time source.

## Pros and Cons of Solana

### Features of Solana

Solana's main features you should know:

[Archivers](https://docs.solana.com/proposals/ledger-replication-to-implement#network) – Solana data
is offloaded from validators to a network of nodes. Periodically, they are checked to ensure that
the stored information is correct;  
[Cloudbreak](https://medium.com/solana-labs/cloudbreak-solanas-horizontally-scaled-state-architecture-9a86679dcbb1)
– a horizontally scalable database of accounts;  
[Gulf Stream](https://medium.com/solana-labs/gulf-stream-solanas-mempool-less-transaction-forwarding-protocol-d342e72186ad)
– a protocol that determines when and how transactions exchange;  
[Pipeline](https://medium.com/solana-labs/pipelining-in-solana-the-transaction-processing-unit-2bb01dbd2d8f)
– a transaction processing unit to optimize validation;  
[Proof-of-History (PoH)](https://docs.solana.com/cluster/synchronization#more-on-proof-of-history) –
an addition to the Proof-of-stake consensus algorithm that improves the efficiency and capacity of
Solana's network;  
[Sealevel](https://medium.com/solana-labs/sealevel-parallel-processing-thousands-of-smart-contracts-d814b378192)
– parallel smart contracts run-time;  
[Tower BFT](https://docs.solana.com/implemented-proposals/tower-bft) – an algorithm that takes
advantage of synchronized clocks. Tower BFT uses PoH as its cryptographic clock;  
[Turbine](https://docs.solana.com/cluster/turbine-block-propagation) – a protocol that simplifies
data transfer to blockchain nodes by breaking data into smaller packets.

### Why do users choose Solana?

So, what are the advantages of Solana? One key advantage is the incredible transaction processing
speed, thousands of times faster than a popular blockchain like Ethereum.

The network's native token is the SOL token, which, for the most part, is essential for steaming and
paying commissions. The not-insignificant thing is that the blockchain burns up half of the
commission amount to maintain the annual inflation rate of the SOL token. The commission payment
takes place in fractions of the token, which are named “lamports” and correspond to 0.000000001 SOL.
Usually, the commission per transaction does not even exceed $0.05, which makes the Solana
blockchain even more attractive!

👉 If we compare it to Ethereum, Solana has no mempool and hence no delays in processing the
transaction and adding it to the blockchain.

### Advantages

**Solana scalability**  
Solana has excellent scalability capabilities! Thanks to their Turbine Block Propagation, stable
operation of over 1900 validators! The beta launch of the mainnet took place in 2020 and brought
millions of users, with Solana keeping transaction costs low. Isn't this a testament to the system's
excellent scalability?

**Solana is ecofriendly**  
Ethereum, before its transition from the Proof-of-Work consensus algorithm, consumed an enormous
amount of electricity, equal to the consumption of a small country. The massive consumption of
resources harmed the environment. Solana blockchain, as mentioned earlier, uses the Proof-of-Stake
consensus algorithm, which is much more energy-efficient than Proof-of-Work (by about 99%!).

**Solana community**  
The developer community has been gaining popularity since its launch and is now one of the top 5
blockchain communities. Solana community has millions of unique followers and tens of thousands of
people online on platforms like Twitter, Telegram, YouTube, GitHub, and many more!

**Solana programs are written in Rust**  
Rust is a high-level, extremely performant programming language that takes speed from C\C++ and the
safety of functional languages. Hence, programs executed faster compared to JS-based smart-contract
languages and consume fewer resources of the validator node. Another advantage is a compile-time
verification that rust provides out of the box for math operations, types checks, and memory safety.

### Disadvantages

- The main disadvantage of Solana is that it relatively fresh chain and its ecosystem and blockchain
  market coverage are not as high as that of Ethereum, however, Solana has all of the major
  protocols in DeFi, NFT & Gaming, Metaverse, and other areas.

- Solana's development may be considered quite complex compared to Ethereum due to its ideology of
  code and data separation, development language, and lack of development tools. However, control
  over code safety and data security in Solana is much higher for advanced developers when they get
  familiar with it.

- Solana is not stable yet and some downtime and performance spike happens from time to time.

## Solana Architecture Overview

Solana labs created the blockchain architecture to facilitate the development of decentralized,
scalable applications (DApps) and smart contracts. With this in mind, it satisfies all the
characteristics of the famous trilemma blockchain: it's scalable, secure, and decentralized ✓

Data transformation in the Solana blockchain is performed based on the SHA256 hash function. It
obtains a unique combination of characters by using the input data. The network creates the next
hash of data based on the received transaction information.

The way described above creates a continuous, long chain of hashed transactions, which is relatively
easy to understand and verify. 👌 This data algorithm is what makes transaction processing so
complex.

# Solana Program Development

## Solana Program File structure

Most Rust-based programs adhere to the following architecture (File - Description)👇:

- lib.rs - Registering modules
- entrypoint.rs - Entrypoint to the program
- instruction.rs - Program API, (de)serializing instruction data
- processor.rs - Program logic
- state.rs - Program objects, (de)serializing state
- error.rs - Program-specific errors

In fact, one lib.rs module is enough, but it’s good practice to split the program into the modules
listed above 🤓.

### Data Layout

Unlike many other chains in Solana, data is stored as raw bytes. To keep data consistent, all the
information is serialized and deserialized via borsh library with the corresponding layout:

![Solana Data Layout](/assets/data-layout.png)

## Accounts

Accounts can be thought of as collections of related information, similar to a file in an operating
system. Accounts are both used by and owned by programs, and a single program can own many different
accounts.

Like a file, accounts can store arbitrary kinds of data (e.g. integers, strings, pubkeys) as well as
SOL. Accounts also have metadata that describes who is allowed to access its data and how long the
account can live. Anyone can read or credit an account, but only the account owner can debit it or
modify its data.

Accounts are created by simply generating a new keypair and registering its public key with the
System Program. Each account is identified by its unique address (aka public key), the same kind
which is used to identify your wallet e.g. Phantom Wallet.

Each account holds the following metadata in `AccountInfo` struct:

```
pub struct AccountInfo<'a> {
    pub key: &'a Pubkey,
    pub lamports: Rc<RefCell<&'a mut u64>>, // the number of lamports owned by this account
    pub data: Rc<RefCell<&'a mut [u8]>>, // raw data byte array stored by this account
    pub owner: &'a Pubkey, // the program owner
    pub rent_epoch: u64,
    pub is_signer: bool,
    pub is_writable: bool,
    pub executable: bool,
}
```

### Executable Accounts (Program Account)

When you deploy an upgradeable program, both the Program Account and the Program Data Account are
created. The Program Account is created first and only this program account is marked as executable.

Features of the Program Account:

- The code of a program is first written in Rust or C/C++ and then compiled into byte code via the
  LLVM compiler infrastructure;
- Store program ID (the unique public key associated with the deployed program);
- It is marked as executable and owned by the BPF Loader program
  (BPFLoaderUpgradeab1e11111111111111111111111 for upgradeable programs);
- This account is relatively small and doesn’t store the actual bytecode. Instead, it serves as an
  entry point that references the Program Data Account, where the actual bytecode resides.
- When a user or another program interacts with the Program Account, the BPF Loader program is
  invoked, which looks up the Program Data Account to fetch the bytecode and execute it.
- Program Upgrades
  - If the program is marked as upgradeable, the Program Data Account also stores the upgrade
    authority. When an upgrade is initiated, the transaction must be signed by the upgrade
    authority’s private key.
  - If the upgrade authority is set to `None`, the program becomes immutable, and the Program Data
    Account can no longer be modified.

### Program Data Account

The Program Data Account is where the actual BPF bytecode (the compiled program code) is stored.

Features of the Program Data Account:

- The Program Data Account is linked to the Program Account but does not have its own public-facing
  Program ID.
- Owned by the BPF Loader program (not the System Program or any user account).
- Stores the executable bytecode (the compiled BPF program) and any associated metadata, such as the
  upgrade authority.
- When a user or another program interacts with the Program Account, the BPF Loader program is
  invoked, which looks up the Program Data Account to fetch the bytecode and execute it.
- Since it’s not directly executed, it doesn’t need the executable flag.
- When an upgrade is initiated, the upgrade authority must sign the transaction to authorize the
  change in the Program Data Account. The Program Account (with the same Program ID) continues to
  point to the updated Program Data Account, which contains the new bytecode.

### System Accounts (Accounts with private keys)

System accounts are standard Solana accounts used for storing SOL and paying for transaction fees.

• These accounts are created by the System Program and are owned by the System Program
(11111111111111111111111111111111)  
• These are basic accounts that don’t have any special program ownership or custom data associated
with them  
• Primarily used to hold SOL and handle transactions between users.

### State Accounts

State accounts are created by the program to store specific application data (state) and are owned
by the program.

- State accounts may or may not use a Program-Derived Address (PDA).
- If a state account is created without a PDA, it would typically be funded by the user’s wallet or
  another Solana account. In this case, the private key is needed to sign and fund the transaction
  that creates the account on-chain.

  Example: A user creates an account to store their custom program data, and they sign the
  transaction using their private key.

- If the state account is created as a PDA, the program derives the account address
  deterministically using seeds and the program ID.

### Program Derived Accounts

A Program-Derived Address (PDA) is a special kind of Solana account that is derived
deterministically using a `seed` and a `program ID`.

- PDAs do not have private keys, which means they are entirely controlled by the program that
  derived them, and only the program can authorize actions on them.
- They are used to represent program-controlled accounts, such as escrow accounts, vaults, or, in
  the case of ATAs, token accounts for specific users.
- When a PDA is created, it is typically funded by a user’s wallet (or another account) that holds
  SOL.
- An Associated Token Account (`ATA`) is a PDA that is specifically derived for a user’s wallet
  address and a token mint under the SPL Token Program. The ATA is deterministically derived based
  on a combination of:
  - The user’s wallet address (public key),
  - The token mint address (the type of token, e.g., USDC, USDT, etc.),
  - The SPL Token Program ID.
  - This ensures that for every unique user-token pair, the SPL Token Program can always derive the
    same Associated Token Account without needing a private key.

## Program Tests

You can test programs on Solana in different ways, through

1. routine calls to the necessary functions (standard unit testing);
2. TestValidator and rpc_client calls;
3. Program_test;
4. JavaScript/TypeScript client;
5. deployment and calling with rpc_api.

Associated token accounts (ATAs) In Solana, there is only one Token Program, which handles the
creation of new tokens, that are not actually executable unlike many other chains, and just storage
with token metadata. So one of the most important examples of PDA is a so-called associated token
account (ATA). Such accounts are used for storing user the balance of concrete SPL-token (token
mint) for the concrete User. In other words, the ATA for a given wallet address is simply a
program-derived account consisting of the wallet address itself and the token mint.

### 1. Standard Unit Testing

You can call the program functions in Solana directly from a classic child module (labeled mod test
with macro #[cfg(test)]). All you need to do is correctly create and initialize all variables
(program_id, accounts, instructions) and then pass them to the function under test and check the
result. ☝️ In this case, we do not need to work with transactions because we work directly with the
account. The disadvantage of this way of testing is that we can't test inter-program interactions.

e.g. sanity test in
[career_booster_beginner/s1_hello_solana/src/lib.rs](/career_booster_beginner/s1_hello_solana/src/lib.rs)

### 2. TestValidator and rpc_client calls (To be continued)

You can call the program functions in Solana via the RPC client of the test validator derived from
the Solana test validator crate. The call will take place on the local node of the validator.

### 3. Program_test (Mostly used in integration tests)

You can call program functions in Solana inside a local instance of the runtime, which saves its
state for the duration of the test and allows you to send multiple transactions. The runtime
environment instance is in the solana_program_test crate. You can configure your local instance
interactively.

example in [s1_hello_solana/tests/lib.rs](/career_booster_beginner/s1_hello_solana/tests/lib.rs)

### 4. JavaScript/TypeScript client (On Testnet / Devnet / Localhost)

Scripts in JavaScript or TypeScript can be used to interact with the program. An example would be
inside

[anchor-cli/scripts](/buffalo_joe/rust_solana/anchor-cli/scripts)

### 5. Deployment and calling with rpc_api (On Testnet / Devnet)

You can call the functions of a Solana program via rpc_api since Solana nodes accept HTTP requests.
Consider this method as a pre-release testing method.

# Career Booster Beginner Course

## Introduction to Solana

😎 Solana is an energy-efficient Tier 1 blockchain with high transaction processing speeds and the
ability to scale stably. It launched in 2017 with the then-innovative Proof-of-History (PoH)
consensus mechanism, which provides one of the highest transaction processing speeds per second
(more than 700,000 transactions per second ✌). The following year, the development team released a
testnet, and they released the mainnet in 2020.

### Brief intro to its Architecture

Solana labs created the blockchain architecture to facilitate the development of decentralized,
scalable applications (DApps) and smart contracts. With this in mind, it satisfies all the
characteristics of the famous trilemma blockchain: it's scalable, secure, and decentralized ✓

Data transformation in the Solana blockchain is performed based on the SHA256 hash function. It
obtains a unique combination of characters by using the input data. The network creates the next
hash of data based on the received transaction information.

The way described above creates a continuous, long chain of hashed transactions, which is relatively
easy to understand and verify. 👌 This data algorithm is what makes transaction processing so
complex.

### Ethereum vs Solana

Ethereum is currently the most prominent blockchain. It hosts more decentralized applications than
any other blockchain ecosystem. However, due to Ethereum's low bandwidth processing capacity of
about 15 transactions per second, the vast enthusiasm around the project makes gas prices
exorbitant.

👉 In contrast, the Solana blockchain combines several new protocols, solving the trilemma of
scalability, security, and decentralization. It takes 0.4 seconds to process a block, and as
hardware capacity increases, so does the speed of the entire network.

This difference in speed is due to the difference in the consensus algorithm.

### Solana Consensus Algorithm

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

## Solana Development Overview

### Solana Program File structure

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

# Solana Learning Journey

This repo is showcases learning resources I found for Solana and tracks my learning journey. I will
be updating this repo with my progress and resources I find useful. Any feedback or suggestions are
welcome. If any of the resources are outdated or incorrect, please let me know.

## Prioritized Solana and Rust Learning Resources

[Beginner Solana and Rust course](https://careerbooster.io/courses/full-solana-and-rust-programming-course-for-beginners)  
[Advanced Solana and Rust course](https://careerbooster.io/courses/rust-solana-advance-development-course)
[Build on Solana](https://www.risein.com/courses/build-on-solana)

## Other Learning Resources

### Solana

[Solana bootcamp Youtube playlist](https://www.youtube.com/playlist?list=PLilwLeBwGuK6NsYMPP_BlVkeQgff0NwvU)  
[Buffalo Joe YouTube channel(Rust on Solana)](https://www.youtube.com/@CodingCrypto/)  
[SolAndy Soldev Youtube playlist](https://www.youtube.com/playlist?list=PLmAMfj0qP2wwfnuRJQge2ss4sJxnhIqyt)  
[SolDev - Official Solana Learning Courses](https://solana.com/developers)
[FreeCodeCamp](https://web3.freecodecamp.org/solana)

### Rust

[Buffalo Joe Rust Beginner Tutorial](https://www.youtube.com/playlist?list=PLUBKxx7QjtVnXD7-u8iIVeIdQXmYRptp-)  
[Buffalo Joe Rust Intermediate Tutorial](https://www.youtube.com/playlist?list=PLUBKxx7QjtVk9cVT9VaTtoDKivyWuLZZf)  
[Let's Get Rusty Learning Guide](https://learn.letsgetrusty.com/)
[Rust Programming Language Book](https://doc.rust-lang.org/book/)

## Advanced - Solana Programs

[Create Twitter on Solana guide from Loris Leiva](https://lorisleiva.com/create-a-solana-dapp-from-scratch/)

Programs to learn from (in order of complexity from easiest to hardest)  
[Anchor starter with 3 counter programs](https://github.com/solana-developers/anchor-starter)  
[Anchor example contracts](https://github.com/tgaye/AnchorExampleContracts/)  
[Solana program examples](https://github.com/solana-developers/program-examples)  
[Solana program library](https://github.com/solana-labs/solana-program-library)  
[Solana Open-Source list](https://github.com/StockpileLabs/awesome-solana-oss)

[Solana programming resources](https://github.com/SolanaNatives/Solana-Programming-Resources)  
[Web3 builders Alliance](https://web3builders.dev/builders)

## Learning flow

![Solana Learning Flow](/assets/flow.png)

## Other references

[The Rust Programming Language](https://doc.rust-lang.org/book/)
[Rust By Example](https://doc.rust-lang.org/rust-by-example/)
[Rustlings](https://github.com/rust-lang/rustlings/)
[Rust makes you feel like a genius](https://www.youtube.com/watch?v=0rJ94rbdteE)
[CS156A](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/book)
[Why Rust is a significant development in programming languages](https://www.youtube.com/watch?v=IwjlCxwcuIc)
[Considering Rust](https://www.youtube.com/watch?v=DnT-LUQgc7s)
[Traits and trait objects - more than just interfaces - Rust Community Stuttgart](https://www.youtube.com/watch?v=izXf9-CTAfc)
[Tour of Rusts standard library Traits](https://github.com/pretzelhammer/rust-blog/blob/master/posts/tour-of-rusts-standard-library-traits.md)
[The curse of strong typing](https://fasterthanli.me/articles/the-curse-of-strong-typing)
[A Gentle Introduction To Rust](https://stevedonovan.github.io/rust-gentle-intro/1-basics.html)
[Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/intro.html)
[Rust for C++ Programmers](https://github.com/nrc/r4cppp)

# Solana

TODO: Summarize things learnt and create a summary below

## Accounts

![Solana Accounts Flow](/assets/solana-account.png)

```

## Solana CLI

- Init a new Solana program for development

```

$ cargo init --lib <program_name>

```

- Compile a program to generate `target/` directory and `program.so` file

```

$ cd <directory with cargo.toml> $ cargo build-bpf

```

- Deploy program (You must have compiled a program using the command above)

```

$ cd <directory with .so file, usually in target/deploy/> $ solana program deploy program.so

```

- Get program ID from keypair

```

$ solana address -k target/deploy/hello_world_example-keypair.json
6RTLRg3mjopTSDCfEPjEwT2siszbYE7EcafkXw3mT2rS

```

## Anchor CLI (Work in progress)

- Init a new Anchor program

```

anchor init <project_name>

```

- create a new program

```

cd <project_name> anchor new <program_name>

```

- Build the program

```

anchor build

```

- Test the program

```

anchor test

```

```

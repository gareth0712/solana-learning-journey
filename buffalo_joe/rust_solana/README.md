# Rust on Solana

This is a course hosted by Buffalo Joe re Rust on Solana. This course is more focused on CLI and
program interaction with Solana.

## CLI for Solana program

- Init a new Solana program for development

```
$ cargo init --lib <program_name>
```

- Compile a program to generate `target/` directory and `program.so` file

```
$ cd <directory with cargo.toml>
$ cargo build-bpf
```

- Deploy program (You must have compiled a program using the command above)

```
$ cd <directory with .so file, usually in target/deploy/>
$ solana program deploy program.so
```

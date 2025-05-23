# TCP From Scratch in Rust

This project is a manual implementation of TCP, written entirely in Rust, using raw sockets and no external crates.

It demonstrates how TCP/IP works under the hood by building packets from scratch and sending them over the network. Everything from IP headers to TCP flags and checksums is implemented by hand.

---

## Why This Project?

Most developers use high-level libraries to handle networking. This project takes a different approach:

- How does TCP actually work?
- What does a SYN packet look like at the byte level?
- Can you build a working TCP segment manually?

By writing everything from scratch, this project explores fundamental networking concepts and system-level programming with Rust.

---

## Features

- Raw socket creation via libc
- Manual construction of IPv4 and TCP headers
- Custom checksum calculation
- Sends TCP SYN packets from scratch
- No networking crates or standard library TCP tools used
- Fully reproducible with Nix Flakes

---

## Dev Environment

This project uses Nix for a clean, reproducible environment.

To enter the development shell:

```bash
nix develop

# Rust Blockchain Project

## Introduction

This project aims to develop a blockchain implementation from scratch using the Rust programming language. The primary goal is to create a blockchain system for educational purposes, allowing individuals to understand the fundamentals of blockchain technology and its underlying principles.

## Features

- **Blockchain Implementation:** The project provides a basic implementation of a blockchain, including blocks, mining, and validation functionalities.
- **Proof of Work:** Utilizes a proof-of-work consensus mechanism for block mining.
- **Concurrency:** Utilizes multi-threading to simulate concurrent mining of blocks.
- **Educational Purpose:** Designed with the intention of being a learning resource for understanding blockchain concepts and Rust programming.

## Usage

To use this program, follow these steps:

1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Ensure you have Rust installed on your machine.
4. Run the program using the `cargo run` command.

## Code Overview

The main functionality of the program is divided into two modules:

- **block:** Contains the definition and implementation of the `Block` struct, which represents individual blocks in the blockchain.
- **blockchain:** Contains the definition and implementation of the `BlockChain` struct, which manages the blockchain, including adding blocks, validating the chain.

The `main` function initializes the blockchain and spawns multiple threads to simulate concurrent mining of blocks. Once the specified number of blocks is mined, the program outputs the resulting blockchain.

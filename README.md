# SPL Staking on Solana

This project enables staking of SPL tokens on the Solana blockchain. It is developed using the Anchor framework, which simplifies the creation of smart contracts (programs) on Solana efficiently and securely.

## Main Features

- SPL token staking.
- Developed with Anchor.
- Modular and easy-to-extend structure.

## Prerequisites

- [Node.js](https://nodejs.org/) >= 16
- [Yarn](https://yarnpkg.com/) or [npm](https://www.npmjs.com/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://book.anchor-lang.com/chapter_1/installation.html)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/franRappazzini/spl-staking.git
   cd spl-staking
   ```
2. Install Node.js dependencies:
   ```bash
   yarn install
   # or
   npm install
   ```
3. Update Rust dependencies (if needed):
   ```bash
   rustup update
   ```
4. Install Anchor CLI:
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked
   ```

## Usage

### 1. Build the program

```bash
anchor build
```

### 2. Deploy to localnet

```bash
anchor deploy
```

### 3. Run tests

```bash
anchor test
```

## Project Structure

- `programs/`: Rust program source code.
- `tests/`: Automated tests.

## Resources

- [Anchor Documentation](https://book.anchor-lang.com/)
- [Solana Docs](https://docs.solana.com/)

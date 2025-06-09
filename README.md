# Partizee

Partizee is a CLI tool for creating and deploying full-stack Partisia dApps. It streamlines the process of managing accounts, wallets, and deploying decentralized applications on the Partisia blockchain.

## Features
- Generate a template projects
- Create and manage Partisia blockchain accounts and wallets
- Deploy and manage full-stack dApps
- Interactive menus for account selection and creation

## Installation

### Quick Install

You can use the provided install script to build and install the CLI to your local bin directory:

```sh
./install.sh
```

This will build the project and copy the `partizee` binary to `~/.local/bin/partizee`.  
Make sure `~/.local/bin` is in your `PATH`:

```sh
export PATH="$PATH:$HOME/.local/bin"
```

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Partisia Blockchain CLI tools](https://partisiablockchain.com/)

### External Documentation

- [cargo-partisia-contract](https://gitlab.com/partisiablockchain/language/cargo-partisia-contract)
- [partisia-cli](https://gitlab.com/partisiablockchain/language/partisia-cli)

### Build from Source

Clone the repository and build the CLI:

```sh
git clone https://github.com/yourusername/partizee.git
cd partizee
cargo build --release
```

The binary will be located in `target/release/partizee`.

## Usage

Run the CLI tool:

```sh
cargo partizee [COMMAND] [OPTIONS]
```

## Commands

Partizee provides several commands for managing your Partisia dApp workflow:

### Project Scaffolding

- `partizee new [OPTIONS]`
  - Scaffold a new project.
  - Options:
    - `-i`, `--interactive` — Use interactive menu to create a new dapp.
    - `--name <NAME>` — Dapp name.
    - `-o`, `--output-dir <DIR>` — Specify a custom output directory for the new project.
    - `-z`, `--zero-knowledge` — Scaffold a zero-knowledge dapp (reserved for future use).

### Compilation

- `partizee compile [OPTIONS]`
  - Compile your dapp.
  - Options:
    - `-i`, `--interactive` — Use interactive menu to compile.
    - `-f`, `--files <FILE>` — Specify specific files to compile.
    - `-b`, `--build-args <ARGS>` — Additional arguments passed to `cargo build`.
    - `-a`, `--additional-args <ARGS>` — Additional arguments for the compile CLI command as specified in the cargo-partisia-contract readme.

### Deployment

- `partizee deploy [OPTIONS]`
  - Deploy your dapp.
  - Options:
    - `-i`, `--interactive` — Use interactive menu to deploy.
    - `-c`, `--chain <NETWORK>` — Select mainnet or testnet (defaults to testnet if not specified).
    - `-n`, `--names <NAMES>` — Names of the contracts to deploy (can specify multiple).
    - `-d`, `--deploy-args <ARGS>` — Contract name followed by its arguments, e.g. `--deploy-args MyContract arg1 arg2`.
    - `-a`, `--account <PATH>` — Path to the account pk file.

### Profile Management

- `partizee account create [OPTIONS]`
  - Create a new blockchain account.
  - Options (all can be combined as needed):
    - `-i`, `--interactive` — Use interactive menu to create account.
    - `-n`, `--name <NAME>` — Name of the account.
    - `-n`, `--network <NETWORK>` — Network to create account on.
    - `-p`, `--path <PATH>` — Path to a .pk file to use for account
    - `-a`, `--address <ADDRESS>` — Profile address (optional if path to a valid pk file or a valid private key is provided)
    - `-k', '--private-key <PRIVATE_KEY>` — private-key to be used

- `partizee account show [OPTIONS]`
  - Show account details.
  - Options (same as above):
    - `-i`, `--interactive` — Use interactive menu to select and show an account.
    - `-n`, `--name <NAME>` — Name of the account.
    - `-n`, `--network <NETWORK>` — Network.
    - `-p`, `--path <PATH>` — Path to a .pk file to use for account
    - `-a`, `--address <ADDRESS>` — Profile address (optional if path to a valid pk file or a valid private key is provided)
    - `-k', '--private-key <PRIVATE_KEY>` — private-key to be used

- `partizee account mint-gas [OPTIONS]`
  - Mint gas for a testnet account.
  - Options (same as above):
    - `-i`, `--interactive` — Use interactive menu to select the account.
    - `-n`, `--name <NAME>` — Name of the account.
    - `-n`, `--network <NETWORK>` — Network.
    - `-p`, `--path <PATH>` — Path to a .pk file to use for account
    - `-a`, `--address <ADDRESS>` — Profile address (optional if path to a valid pk file or a valid private key is provided)
    - `-k', '--private-key <PRIVATE_KEY>` — private-key to be used

---

For more details on each command and its options, run:

```sh
partizee --help
```

You can copy and paste this into your README, replacing or expanding the "Common Commands" section. This gives users a clear overview of what each command does and what options are available.

### Example

```sh
cargo partizee account create --net=testnet
```

## Configuration

Partizee stores private keys and account information securely in your workspace. You can specify network, address, and private key via command-line options or configuration files.

## Development

### Running Tests

```sh
cargo test
```

### Code Structure

- `src/commands/account.rs` — Profile and wallet management logic
- `src/utils/` — Utility functions for file system navigation, menus, and more

## Contributing

Contributions are welcome! Please open issues or pull requests for bug fixes, features, or documentation improvements.

## License

[MIT](LICENSE)

## Disclaimer

This tool is provided as-is. Use at your own risk. Always keep your private keys secure.

---

For more information, see the [Partisia Blockchain documentation](https://partisiablockchain.com/).

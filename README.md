# Partizee

Partizee is a CLI tool for creating and deploying full-stack Partisia dApps. It streamlines the process of managing accounts, wallets, and deploying decentralized applications on the Partisia blockchain.

## Features
- Generate a template projects
- Create and manage Partisia blockchain profiles and wallets
- Deploy and manage full-stack dApps
- Interactive menus for profile selection and creation

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
partizee [COMMAND] [OPTIONS]
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
    - `-a`, `--additional-args <ARGS>` — Additional arguments for the compile CLI command.

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

Partizee uses a profile-based system for managing blockchain identities. Each profile contains:
- Network (mainnet/testnet)
- Address
- Private key
- Path to private key file

Profiles can be created and managed in several ways:

1. **Default Profile**: If no profile exists, Partizee will automatically:
   - Create a new wallet if none exists
   - Create a new account on testnet
   - Store the private key file in your workspace

2. **Profile Creation Options**:
   - From an existing private key file
   - From an address and private key pair
   - From just a private key (address will be derived)
   - From just an address (will search for matching private key file)
   - Interactively through the menu system

3. **Profile Operations**:
   - Show account details and balance
   - Mint gas (testnet only)
   - Update private key
   - Update network
   - Update address

4. **Profile Configuration**:
   You can specify profile details through:
   - Command line options
   - Configuration files
   - Interactive menus
   - Environment variables

### Example Usage

```sh
# Create a new project
partizee new my-dapp

# Deploy using a specific profile
partizee deploy --chain testnet --deploy-args 00d277aa1bf5702ab9fc690b04bd68b5a981095530
```

---

For more details on each command and its options, run:

```sh
partizee --help
```

You can copy and paste this into your README, replacing or expanding the "Common Commands" section. This gives users a clear overview of what each command does and what options are available.

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

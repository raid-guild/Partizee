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

This will build the project and copy the `partizee` binary to `~/.local/bin/partizee` and add the tool to your local path.  
You can make sure `~/.local/bin` is in your `PATH` by executing:

```sh
export PATH="$PATH:$HOME/.local/bin"
```

### Prerequisites

Ensure the following dependencies are installed before using Partizee:

- **Rust (version 1.86.0 required):** Install via [Rustup](https://rustup.rs/).  
  _Note: You must use Rust v1.86.0. Newer versions may result in compiler errors._
- **WASM Target for Rust:**  
  After installing Rust, add the required target:
  ```sh
  rustup target add wasm32-unknown-unknown
  ```
- **Git:** Required for contract compilation. [Download Git](https://git-scm.com/downloads)
- **OpenJDK 17:** Required for contract compilation and related tooling. [Install OpenJDK 17](https://openjdk.org/install/)
- **Partisia Contract CLI Tool:** Install the official CLI tools as described in the [Partisia Blockchain documentation](https://partisiablockchain.gitlab.io/documentation/smart-contracts/install-the-smart-contract-compiler.html).
- **Windows Only:** If you are on Windows, ensure [Visual Studio with C++](https://visualstudio.microsoft.com/downloads/) is installed for contract compilation.

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

then run the install script
```sh
bash ./install.sh
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
  - Scaffold a new project. If no flags are passed and no data the interactive menu will open automatically.
  - Options:
    - `-i`, `--interactive` — Use interactive menu to create a new dapp.
    - `--name <NAME>` — Dapp name.
    - `-o`, `--output-dir <DIR>` — Specify a custom output directory for the new project.
    - `-z`, `--zero-knowledge` — Scaffold a zero-knowledge dapp (reserved for future use).

### Compilation

- `partizee compile [OPTIONS]`
  - Compile your dapp. If no flags are passed and no data the interactive menu will open automatically.
  - Options:
    - `-i`, `--interactive` — Use interactive menu to compile.
    - `-f`, `--files <FILE>` — Specify specific files to compile.
    - `-b`, `--build-args <ARGS>` — Additional arguments passed to `cargo build`.
    - `-a`, `--additional-args <ARGS>` — Additional arguments for the compile CLI command.

### Deployment

- `partizee deploy [OPTIONS]`
  - Deploy your dapp. If no flags are passed and no data the interactive menu will open automatically.
  - Options:
    - `-i`, `--interactive` — Use interactive menu to deploy.
    - `-c`, `--chain <NETWORK>` — Select mainnet or testnet (defaults to testnet if not specified).
    - `-n`, `--names <NAMES>` — Names of the contracts to deploy (can specify multiple).
    - `-d`, `--deploy-args <CONTRACT NAME> <ARGS>` — Contract name followed by its arguments, e.g. `--deploy-args MyContract arg1 arg2`.  for multiple contract deployments you can use mutliple flags `-d contract-name1 arg1 arg2 -d contract-name2 arg3 arg4` or you can put the arguments all in one string seperated by the contract name `-d contract-name1 arg1 arg2 countract-name2 arg3 arg4`
    - `-a`, `--account <PATH>` — Path to the account pk file.

### Profile Management Commands

- `partizee profile create [OPTIONS]`
  - Create a new blockchain profile (account). If no options are provided, the interactive menu will open.  if no wallet exists on your machine it will ask to create a new wallet before the account can be made.

  - Options:
    - `-i`, `--interactive` — Use interactive menu to create a new profile.
    - `-n`, `--network <NETWORK>` — Specify the network (mainnet/testnet).

- `partizee profile show [OPTIONS]`
  - Show details for a blockchain profile/account.
  - Options:
    - `-i`, `--interactive` — Use interactive menu to select and show a profile.
    - `-n`, `--network <NETWORK>` — Specify the network.
    - `-a`, `--address <ADDRESS>` — Specify the account address.

- `partizee profile mint-gas [OPTIONS]`
  - Mint testnet gas for a profile/account (testnet only).
  - Options:
    - `-i`, `--interactive` — Use interactive menu to select a profile.
    - `-n`, `--network <NETWORK>` — Specify the network.
    - `-a`, `--address <ADDRESS>` — Specify the account address.
    - `-k`, `--private-key <PRIVATE_KEY>` — Specify the private key.
    - `-p`, `--path <PATH>` — Path to the account pk file.

### Example Usage

```sh
# Create a new project
partizee new my-dapp
# Create a wallet/account
partizee profile create
# Compile contracts
partizee compile
# Deploy using a specific profile
partizee deploy --chain testnet --deploy-args counter 00d277aa1bf5702ab9fc690b04bd68b5a981095530
# or with the interactive menu
partizee deploy
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

## Contributing

Contributions are welcome! Please open issues or pull requests for bug fixes, features, or documentation improvements.

## License

[MIT](LICENSE)

## Disclaimer

This tool is provided as-is. Use at your own risk. Always keep your private keys secure.

---

For more information, see the [Partisia Blockchain documentation](https://partisiablockchain.com/).

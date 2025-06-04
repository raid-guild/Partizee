use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "cargo", bin_name = "cargo")]
pub(crate) enum Cargo {
    Partizee(Arguments),
}

#[derive(Args)]
#[clap(
    author,
    version,
    long_about = "\nPartizee \nScaffolds builds and deploys new partisia dapps.",
    about = "\nPartizee \nScaffolds builds and deploys new partisia dapps"
)]
pub(crate) struct Arguments {
    #[clap(subcommand)]
    pub(crate) commands: Commands,
}
#[derive(Args, Clone)]
#[clap(disable_help_flag = true)]
pub struct CliArgs {
    #[clap(help = "no messaages printed to stdout", short = 'q', long = "quiet")]
    quiet: bool,
    #[clap(
        help = "Url specifying the location to retrieve the partizee JAR from. If not given, a user configuration file in\n\
                    ~/.partizee/config.toml or default values will be used.\n\
                    Uses netrc for authentication.\n\
                    Example usage:\n\
                     --use https://gitlab.com/api/v4/groups/12499775/-/packages/maven/com/partisiablockchain/language/partisia-cli/4.1.0/partisia-cli-4.1.0-jar-with-dependencies.jar",
        short = 'u',
        long = "use"
    )]
    pub(crate) url: Option<String>,
    #[clap(
        help = "Print usage description of the command.",
        short = 'h',
        long = "help"
    )]
    pub(crate) help: bool,
    #[clap(
        help = "Print version of the Partisia Cli",
        short = 'V',
        long = "version"
    )]
    pub(crate) version: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "create a new dapp")]
    New {
        #[clap(help = "dapp name", value_parser)]
        name: Option<String>,
        #[clap(help = "output directory", short = 'o', long = "output-dir")]
        output_dir: Option<String>,
        #[clap(
            help = "scaffold a zero-knowledge dapp",
            short = 'z',
            long = "zero-knowledge"
        )]
        zero_knowledge: bool,
    },

    #[clap(about = "compile your dapp")]
    Compile {
        #[clap(
            long = "file",
            short = 'f',
            help = "Specify specific files to compile",
            num_args = 1..,
            allow_hyphen_values = true
        )]
        files_to_compile: Option<String>,

        #[clap(
            long = "build-args",
            short = 'b',
            help = "Additional arguments that will be passed along to cargo build, \n\
                see cargo build --help for details.",
            num_args = 1..,
            allow_hyphen_values = true
            )]
        build_args: Option<Vec<String>>,

        #[clap(
            long = "additional-args",
            short = 'c',
            help = "Additional arguments that will be passed along to compile cli command",
            num_args = 1..,
            allow_hyphen_values = true
            )]
        additional_args: Option<Vec<String>>,
    },

    #[clap(about = "deploy your dapp")]
    Deploy {
        #[clap(
            help = "select mainnet or testnet, if no thing is specified, it defaults to testnet",
            short = 'n',
            long = "net"
        )]
        custom_net: Option<String>,
        #[clap(
            help = "enter the path to the contract to deploy",
            short = 'p',
            long = "path"
        )]
        custom_path: Option<String>,
        #[clap(help = "path to the project root", short = 'r', long = "root")]
        custom_root: Option<String>,
        #[clap(
          help = "additional deployer arguments to pass to deploy cli command",
          short = 'd',
          long = "deployer-args",
          num_args = 0..,
          allow_hyphen_values = true)]
        custom_deployer_args: Option<Vec<String>>,
    },
    #[clap(about = "create a new account")]
    Account {
        #[clap(subcommand)]
        commands: AccountSubcommands,
    },
}

#[derive(Args, Debug)]
pub struct AccountSharedArgs {
    #[clap(
        help = "use interactive menu to create account",
        short = 'i',
        long = "interactive"
    )]
    pub(crate) interactive: bool,
    #[clap(help = "name of the account", short = 'n', long = "name")]
    pub(crate) name: Option<String>,
    #[clap(help = "network to create account on", short = 'n', long = "network")]
    pub(crate) network: Option<String>,
    #[clap(help = "path to the account", short = 'p', long = "path")]
    pub(crate) path: Option<String>,
    #[clap(help = "account public key", short = 'k', long = "public-key")]
    pub(crate) public_key: Option<String>,
    #[clap(help = "account address", short = 'a', long = "address")]
    pub(crate) address: Option<String>,
    #[clap(help = "account index", short = 'i', long = "index")]
    pub(crate) account_index: Option<u32>,
}

#[derive(Subcommand)]
pub enum AccountSubcommands {
    #[clap(about = "create a new account", name = "create")]
    AccountCreate {
        #[clap(flatten)]
        shared_args: AccountSharedArgs,
    },
    #[clap(about = "show account", name = "show")]
    AccountShow {
        #[clap(flatten)]
        shared_args: AccountSharedArgs,
    },
    #[clap(about = "mint gas for account", name = "mint-gas")]
    AccountMintGas {
        #[clap(flatten)]
        shared_args: AccountSharedArgs,
    },
}

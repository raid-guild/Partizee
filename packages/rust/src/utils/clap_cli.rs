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
        #[clap(
            help = "use interactive menu to create a new dapp",
            short = 'i',
            long = "interactive"
        )]
        interactive: bool,
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
            help = "use interactive menu to compile",
            short = 'i',
            long = "interactive"
        )]
        interactive: bool,
        #[clap(
            long = "files",
            short = 'f',
            help = "Specify specific files to compile",
            num_args = 1..,
            allow_hyphen_values = true
        )]
        files_to_compile: Option<Vec<String>>,
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
            short = 'a',
            help = "Additional arguments that will be passed along to compile cli command",
            num_args = 1..,
            allow_hyphen_values = true
            )]
        additional_args: Option<Vec<String>>,
    },

    #[clap(about = "deploy your dapp")]
    Deploy {
        #[clap(
            help = "use interactive menu to deploy",
            short = 'i',
            long = "interactive"
        )]
        interactive: bool,
        #[clap(
            help = "select mainnet or testnet, if no thing is specified, it defaults to testnet",
            short = 'c',
            long = "chain"
        )]
        custom_net: Option<String>,
        #[clap(
            help = "enter the names of the contracts to deploy",
            short = 'n',
            long = "names",
            num_args = 1..,
        )]
        contract_names: Option<Vec<String>>,
        #[clap(
            help = "contract name followed by its arguments, e.g. --deploy-args MyContract arg1 arg2",
            long = "deploy-args",
            short = 'd',
            num_args = 1..,
        )]
        deploy_args: Option<Vec<String>>,
        #[clap(help = "path to the account", short = 'a', long = "account")]
        pk_path: Option<String>,
    },

    #[clap(about = "create a new account")]
    Profile {
        #[clap(subcommand)]
        commands: ProfileSubcommands,
    },
}

#[derive(Args, Debug)]
pub struct ProfileSharedArgs {
    #[clap(
        help = "use interactive menu to create account",
        short = 'i',
        long = "interactive"
    )]
    pub(crate) interactive: bool,
    #[clap(help = "name of the account", short = 'n', long = "name")]
    pub(crate) name: Option<String>,
    #[clap(
        help = "network account will be used on",
        short = 'w',
        long = "network"
    )]
    pub(crate) network: Option<String>,
    #[clap(help = "path to pk file", short = 'p', long = "path")]
    pub(crate) path: Option<String>,
    #[clap(help = "account address", short = 'a', long = "address")]
    pub(crate) address: Option<String>,
    #[clap(help = "private key string", short = 'k', long = "private-key")]
    pub(crate) private_key: Option<String>,
}

#[derive(Subcommand)]
pub enum ProfileSubcommands {
    #[clap(about = "create a new account", name = "create")]
    ProfileCreate {
        #[clap(flatten)]
        shared_args: ProfileSharedArgs,
    },
    #[clap(about = "show account", name = "show")]
    ProfileShow {
        #[clap(flatten)]
        shared_args: ProfileSharedArgs,
    },
    #[clap(about = "mint gas for account", name = "mint-gas")]
    ProfileMintGas {
        #[clap(flatten)]
        shared_args: ProfileSharedArgs,
    },
}

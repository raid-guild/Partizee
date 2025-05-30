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
            help = "Specify a specific file to compile"
        )]
        file: Option<String>,

        #[clap(
            long = "build-args",
            short = 'b',
            help = "Additional arguments that will be passed along to cargo build, \n\
                see cargo build --help for details.",
            num_args = 1..,
            allow_hyphen_values = true
            )]
            build_args: Vec<String>,

        #[clap(
            long = "additional-args",
            short = 'c',
            help = "Additional arguments that will be passed along to compile cli command",
            num_args = 0..,
            allow_hyphen_values = true
            )]
        additional_args: Vec<String>,
    },

    #[clap(about = "deploy your dapp")]
    Deploy {
        #[clap(
            help = "select mainnet or testnet, if no thing is specified, it defaults to testnet",
            short = 'n',
            long = "net"
        )]
        net: Option<String>,
        #[clap(
          help = "additional deployer arguments to pass to deploy cli command",
          short = 'd',
          long = "deployer-args",
          num_args = 1..,
          allow_hyphen_values = true)]
        deployer_args: Vec<String>,
    },
}

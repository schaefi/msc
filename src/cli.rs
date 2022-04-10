use clap::{AppSettings, Parser, Subcommand};

/// msc - Open Build Service API client
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
pub struct Cli {
    /// API server URL
    #[clap(long, default_value_t = String::from("https://api.opensuse.org"))]
    pub api_server: String,

    /// OBS project path
    #[clap(long)]
    pub project: String,

    /// OBS user
    #[clap(long)]
    pub user: String,

    /// OBS credentials. Note: specifying on the commandling is insecure
    /// If not provided as an option the credentials for the given user
    /// will be read from stdin
    #[clap(long)]
    pub password: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// get binaries
    GetBinaries {
        /// Package or image name
        package: String,

        /// Distribution name
        dist: String,

        /// Architecture name
        arch: String,

        /// Output directory, will be created if not present
        outdir: String,

        /// Multibuild profile name
        #[clap(long)]
        profile: Option<String>,
    },
    Checkout {
        /// Package or image name
        package: String,

        /// Output directory, will be created if not present
        outdir: String,
    },
}

pub fn parse_args() -> Cli {
    return Cli::parse();
}

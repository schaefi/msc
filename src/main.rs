use rpassword::read_password;
use std::io::Write;

pub mod cli;
pub mod obs_net;
pub mod obs_connect;
pub mod obs_checkout;
pub mod obs_getbinaries;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = cli::parse_args();

    // read access credentials from stdin if not
    // provided insecurely on the command line
    match args.password {
        None => {
            print!("Enter OBS password for user ({}): ", args.user);
            std::io::stdout().flush().unwrap();
            args.password = Some(read_password().unwrap());
        },
        _ => (),
    }

    // operate on provided sub command
    let connect = obs_connect::Obs::new(
        args.api_server,
        args.user,
        String::from(args.password.as_deref().unwrap()),
        args.project
    );
    match &args.command {
        // checkout...
        cli::Commands::Checkout { package } => {
            obs_checkout::checkout(
                connect, obs_checkout::Checkout{
                    package: String::from(package),
                    expand: true
                }, &String::from("outdir")
            ).await?;
        },

        // get-binaries...
        cli::Commands::GetBinaries { package, dist, arch, profile } => {
            obs_getbinaries::getbinaries(
                connect, obs_getbinaries::GetBinaries{
                    package: String::from(package),
                    dist: String::from(dist),
                    arch: String::from(arch),
                    profile: String::from(profile.as_deref().unwrap()),
                    expand: true
                }, &String::from("outdir")
            ).await?;
        },
    }
    Ok(())
}

use crate::obs_connect::Obs;

#[derive(Debug)]
pub struct GetBinaries {
    pub package: String,
    pub dist: String,
    pub arch: String,
    pub profile: String,
    pub expand: bool
}

pub async fn getbinaries(
    connect: Obs, args: GetBinaries, outdir: &String
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO
    println!("{:?}", connect);
    println!("{:?}", args);
    println!("{}", outdir);
    Ok(())
}

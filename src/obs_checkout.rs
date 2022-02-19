use crate::obs_net::build_source_endpoint_url;
use crate::obs_net::send_request;
use crate::obs_net::to_xml;
use crate::obs_connect::Obs;

pub struct Checkout {
    pub package: String,
    pub expand: bool
}

pub async fn checkout(
    connect: Obs, args: Checkout, outdir: &String
) -> Result<(), Box<dyn std::error::Error>> {
    let url = build_source_endpoint_url(
        &connect.api_server, &connect.project, Some(&args.package), true
    );
    let response = send_request(
        &connect.user, &connect.password, &url
    ).await?;
    let response_xml_root = to_xml(&response);

    // TODO
    println!("{:?}", response_xml_root);
    println!("{}", outdir);
    Ok(())
}

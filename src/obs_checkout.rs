use std::fs;
use crate::obs_net::build_source_endpoint_url;
use crate::obs_net::send_request;
use crate::obs_net::fetch_file;
use crate::obs_net::get_text;
use crate::obs_net::to_xml;
use crate::obs_connect::Obs;

pub struct Checkout {
    pub package: String,
    pub expand: bool
}

pub async fn checkout(
    connect: Obs, args: Checkout, outdir: &String
) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(outdir)?;

    // TODO, it would be nice to feed the response into an automatic
    // generated API from the openAPI specs of the buildservice
    // what we do now is sending the request, read into XML and
    // traverse what we get
    let url = build_source_endpoint_url(
        &connect.api_server, &connect.project, Some(&args.package),
        args.expand
    );
    let response = send_request(
        &connect.user, &connect.password, &url
    ).await?;

    const NS: &'static str = "directory";

    let response_xml_root = to_xml(&get_text(response).await?);

    for response_content in response_xml_root.children() {
        let package_name = response_content.attr("name");
        for directory_content in response_content.children() {
            if directory_content.is("entry", NS) {
                let base_file_name = directory_content.attr("name").unwrap();
                let package_file_name = &format!(
                    "{}/{}", package_name.unwrap(), base_file_name
                );
                let fetch_url = build_source_endpoint_url(
                    &connect.api_server, &connect.project, Some(package_file_name), true
                );
                let response = send_request(
                    &connect.user, &connect.password, &fetch_url
                ).await?;
                fetch_file(
                    response, &format!("{}/{}", outdir, base_file_name)
                ).await?;
            }
        }
    }

    Ok(())
}

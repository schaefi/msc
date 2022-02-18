use http_auth_basic::Credentials;
use minidom::Element;
use rpassword::read_password;
use std::io::Write;
use std::io::{Error, ErrorKind};

mod cli;

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
    match &args.command {
        // checkout...
        cli::Commands::Checkout { package } => {
            let url = build_source_endpoint_url(
                &args.api_server, &args.project, Some(package), true
            );
            let response_xml_root = get_xml_response(
                &args.user, args.password.as_ref().unwrap(), &url
            ).await?;

            // TODO
            println!("{:?}", response_xml_root);
        },

        // get-binaries...
        cli::Commands::GetBinaries { package, dist, arch, profile } => {
            let url = build_source_endpoint_url(
                &args.api_server, &args.project, Some(package), true
            );
            println!("{} {} {} {} {:?}", url, package, dist, arch, profile.as_deref());
        },
    }

    Ok(())
}

fn build_source_endpoint_url(
    server: &String, project: &String, package: Option<&String>, expand: bool
) -> String {
    let mut url;
    match package {
        None => {
            url = format!("{}/source/{}", server, project);
        }
        _ => {
            url = format!("{}/source/{}/{}", server, project, package.unwrap());
        }
    }
    if expand {
        url = format!("{}?expand=1", url);
    }
    url
}

async fn get_xml_response(
    user: &String, password: &String, url: &String
) -> Result<Element, Box<dyn std::error::Error>> {
    /*!
    Send GET request to the API server and return
    an XML Element root object
    !*/
    let credentials = Credentials::new(user, password);

    let client = reqwest::Client::builder()
        .build()?;

    let res = client
        .get(url)
        .header("Authorization", credentials.as_http_header())
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::BAD_REQUEST => {
            let request_error = format!(
                "content-length:{:?} server:{:?}",
                res.headers().get(reqwest::header::CONTENT_LENGTH),
                res.headers().get(reqwest::header::SERVER),
            );
            return Err(
                Box::new(Error::new(ErrorKind::InvalidData, request_error))
            )
        },
        status => {
            let request_status = format!("{}", status);
            if request_status != "200 OK" {
                return Err(
                    Box::new(Error::new(ErrorKind::Other, request_status))
                )
            }
        },
    }

    let response_content = res.text().await?;

    let mut xml = String::new();
    xml.push_str("<response xmlns=\"directory\">");
    xml.push_str(&response_content);
    xml.push_str("</response>");

    let root: Element = xml.parse().unwrap();

    Ok(root)
}

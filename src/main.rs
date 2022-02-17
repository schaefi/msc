use http_auth_basic::Credentials;
use error_chain::error_chain;
use minidom::Element;
use rpassword::read_password;
use std::io::Write;

mod cli;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
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

    println!("Hi {:?}", args.password.as_deref());

    // https://api.opensuse.org/source/Virtualization:Appliances:Images:Testing_x86:leap/test-image-luks?expand=1

    let url;
    match &args.command {
        cli::Commands::Checkout { package } => {
            url = build_source_endpoint_url(
                &args.api_server, &args.project, Some(package), true
            );
        },
        // TODO: handle other sub commands
        _ => { url = String::new() },
    }

    // FIXME: url should not be empty
    assert_eq!(url.is_empty(), false);

    println!("{}", url);

    let response_text = send_get_request(
        &args.user, args.password.as_ref().unwrap(), &url
    ).await?;

    // FIXME: handling of auth errors 401, and other that causes html response
    println!("{}", response_text);

    let mut xml = String::new();
    xml.push_str("<response xmlns=\"directory\">");
    xml.push_str(&response_text);
    xml.push_str("</response>");
     
    let root: Element = xml.parse().unwrap();
    println!("{:?}", root);

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

async fn send_get_request(
    user: &String, password: &String, url: &String
) -> Result<String> {
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

    let response_content = res.text().await?;
    Ok(response_content)
}

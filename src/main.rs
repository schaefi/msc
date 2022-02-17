use text_io::read;
use http_auth_basic::Credentials;
// use std::collections::HashMap;
use error_chain::error_chain;
use minidom::Element;

mod cli;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
//async fn main() -> Result<(), Box<dyn std::error::Error>> {
async fn main() -> Result<()> {
    let mut args = cli::parse_args();

    // read access credentials from stdin if not
    // provided insecurely on the command line
    match args.password {
        None => {
            println!("Enter OBS password for user ({}): ", args.user);
            args.password = Some(read!("{}\n"));
        },
        _ => (),
    }

    println!("Hi {:?}", args.password.as_deref());

    let x = get_request().await?;
    //println!("{:?}", x);
    let mut xml = String::new();
    xml.push_str("<response xmlns=\"directory\">");
    xml.push_str(&x);
    xml.push_str("</response>");
     
    let root: Element = xml.parse().unwrap();
    println!("{:?}", root);

    Ok(())
}

//async fn get_request() -> Result<String, Box<dyn std::error::Error>> {
async fn get_request() -> Result<String> {
    let credentials = Credentials::new("foo", "bar");

    let client = reqwest::Client::builder()
        .build()?;

    // Perform the actual execution of the network request
    let res = client
        .get("https://api.opensuse.org/source/Virtualization:Appliances:Images:Testing_x86:leap/test-image-luks?expand=1")
        .header("Authorization", credentials.as_http_header())
        .send()
        .await?;

    // Parse the response body as Json in this case
    let ip = res
        //.json::<HashMap<String, String>>()
        .text()
        .await?;

    //println!("{:?}", ip);

    //Ok(())
    //std::result::Result::Ok(ip)
    Ok(ip)
}

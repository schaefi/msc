use http_auth_basic::Credentials;
use minidom::Element;
use std::io::{Error, ErrorKind};

pub fn build_source_endpoint_url(
    server: &String, project: &String, package: Option<&String>, expand: bool
) -> String {
    /*!
    Create OBS API URL pointng to the source backend
    !*/
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

pub fn to_xml(data: &String) -> Element {
    /*!
    Parse provided data string as XML and return
    minidom root object
    !*/
    let mut xml = String::new();
    xml.push_str("<response xmlns=\"directory\">");
    xml.push_str(&data);
    xml.push_str("</response>");

    let root: Element = xml.parse().unwrap();
    root
}

pub async fn send_request(
    user: &String, password: &String, url: &String
) -> Result<String, Box<dyn std::error::Error>> {
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
    Ok(response_content)
}

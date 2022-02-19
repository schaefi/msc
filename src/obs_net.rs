use http_auth_basic::Credentials;
use minidom::Element;
use std::io::{Error, ErrorKind};
use std::cmp::min;
use std::fs::File;
use std::io::Write;
use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

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

pub async fn get_text(
    response: reqwest::Response
) -> Result<String, Box<dyn std::error::Error>> {
    /*!
    Return text from response
    !*/
    let response_content = response.text().await?;
    Ok(response_content)
}

pub async fn fetch_file(
    response: reqwest::Response, url: &String, filepath: &String
) -> Result<(), Box<dyn std::error::Error>> {
    let total_size = response
        .content_length()
        .ok_or(format!("Failed to get content length from '{}'", &url))?;
    let progress = ProgressBar::new(total_size);

    progress.set_style(ProgressStyle::default_bar()
        .template(
            &format!(
                "{}\n{} [{}] [{}] {}/{} ({}, {})",
                "{msg}",
                "{spinner:.green}",
                "{elapsed_precise}",
                "{wide_bar:.cyan/blue}",
                "{bytes}",
                "{total_bytes}",
                "{bytes_per_sec}",
                "{eta}"
            )
        )
        .progress_chars("#>-"));
    progress.set_message(&format!("Downloading {}", url));

    let mut file = File::create(filepath)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        progress.set_position(new);
    }
    progress.finish_with_message(
        &format!("Downloaded {} to {}", url, filepath)
    );
    return Ok(());
}

pub async fn send_request(
    user: &String, password: &String, url: &String
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    /*!
    Send GET request to the API server and return response object
    !*/
    let credentials = Credentials::new(user, password);

    let client = reqwest::Client::builder()
        .build()?;

    let response = client
        .get(url)
        .header("Authorization", credentials.as_http_header())
        .send()
        .await?;

    match response.status() {
        reqwest::StatusCode::BAD_REQUEST => {
            let request_error = format!(
                "content-length:{:?} server:{:?}",
                response.headers().get(reqwest::header::CONTENT_LENGTH),
                response.headers().get(reqwest::header::SERVER),
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
    Ok(response)
}

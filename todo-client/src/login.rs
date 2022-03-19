/* LOGIN.rs
 *   by Lut99
 *
 * Created:
 *   19 Mar 2022, 21:26:21
 * Last edited:
 *   19 Mar 2022, 22:06:06
 * Auto updated?
 *   Yes
 *
 * Description:
 *   Handles logging in and junk.
**/

use url::Url;

use log::{debug, info};
use reqwest::StatusCode;
use reqwest::blocking::Response;
use todo_spec::api::LoginJson;
use todo_spec::credentials::Credential;

use crate::errors::LoginError as Error;


/***** LIBRARY FUNCTIONS *****/
/// Tentatively logs a user in, just returning true or false depending on whether it was a success.
/// 
/// **Arguments**
///  * `host`: The host to login to.
///  * `cred`: The Credentials to login with.
/// 
/// **Returns**  
/// Whether or not the credentials provided are valid for this host on success, or an Error otherwise.
pub fn test_login(host: Url, cred: Credential) -> Result<bool, Error> {
    info!("Testing credentials at '{}'...", &host);

    // Prepare the request body
    println!("{}", cred.pass());
    let body = LoginJson{ username: cred.user().to_string(), password: cred.pass().to_string() };
    let body = match serde_json::to_string(&body) {
        Ok(body) => body,
        Err(err) => { return Err(Error::SerializeError{ err }); }
    };

    // Compute the path to send the request to
    let url = match host.join("v1/login/test") {
        Ok(url)  => url,
        Err(err) => { return Err(Error::UrlJoinError{ host, path: "v1/login/test".to_string(), err }); }
    };

    // Send the login request
    debug!("Sending test login request to '{}'...", &url);
    let client = reqwest::blocking::Client::new();
    let response: Response = match client.post(&url.to_string())
        .body(body)
        .send()
    {
        Ok(response) => response,
        Err(err)     => { return Err(Error::RequestError{ err }); }
    };

    // Check if any errors occured
    let status = response.status();
    debug!("Host '{}' responsed with status code {} ({})", &host, status.as_u16(), status.canonical_reason().unwrap_or("???"));
    if status != StatusCode::OK && status != StatusCode::NOT_FOUND && status != StatusCode::FORBIDDEN {
        return Err(Error::ResponseError{ status, response: response.text().unwrap_or("<unparseable response>".to_string()) });
    }

    // Done; match the result of the status
    Ok(response.status() == StatusCode::OK)
}

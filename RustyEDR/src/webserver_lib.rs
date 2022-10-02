use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};



const WEB_URL: &str = "test.com";


// POST incident alert to webserver
pub fn incident_alert(auth: &str, payload: &'static str) -> Result<(), String> {

    let full_url = WEB_URL.to_string() + "Blah Blah boring";

    // Debug Section
    if auth == "debug" {
        let output = debug_data();
        return output;
    }


    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));


    let client = reqwest::Client::new();
    let res = client.post(full_url)
    .headers(headers)
    .body(payload)
    .send();
    Ok(())
}

// GET enviroment IPs from webserver
pub fn get_ips() -> Result<(), String> {


    Ok(())
}


// GET configs from webserver
pub fn get_configs() -> Result<(), String> {


    Ok(())
}




// Debug function
fn debug_data() -> Result<(), String> {

    println!("{}", "Bruh moment");

    Ok(())

}
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use reqwest::Client;
use futures::prelude::*;
use serde::{Serialize, Deserialize};


const WEB_URL: &str = "test.com";

#[derive(Serialize)]
pub struct PartialIncident {
    pub name: String,
    pub user: String,
    pub processid: i16,
    pub remoteip: String,
    pub cmdrun: String
}

#[derive(Serialize)]
pub struct FullIncident {
    pub hostname: String,
    pub incident: PartialIncident
}


// POST incident alert to webserver
pub async fn incident_alert(auth: &str, payload: FullIncident) -> String {
    //env_logger::init();
    let endpoint = "BlahBlah/blah";
    let full_url = WEB_URL.to_string() + endpoint; 

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));


    let client = reqwest::Client::new();
    let res = client
        .post(full_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await;

    match res.unwrap().status() {
        reqwest::StatusCode::OK => {
            return "Request delivered".to_string();
        },
        _ => {
            return "Request not delivered".to_string();
        }
    }
}

#[derive(Serialize)]
struct Hearbeat {
    alive: bool
}

// POST incident alert to webserver
pub async fn heartbeat(auth: &str) -> String {
    //env_logger::init();
    let endpoint = "BlahBlah/blah";
    let full_url = WEB_URL.to_string() + endpoint; 

    let payload = Hearbeat {
        alive: true
    };

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));


    let client = reqwest::Client::new();
    let res = client
        .post(full_url)
        .headers(headers)
        .json(&payload)
        .send()
        .await;

    match res.unwrap().status() {
        reqwest::StatusCode::OK => {
            return "beep".to_string();
        },
        _ => {
            return "flatline".to_string();
        }
    }
}

// GET enviroment IPs from webserver
pub async fn get_ips() {

}











// GET configs from webserver
pub fn get_configs() -> Result<(), String> {


    Ok(())
}
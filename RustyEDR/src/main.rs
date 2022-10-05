// Modules
mod webserver_lib;

use std::fs::File;
use std::io::prelude::*;
use webserver_lib::*;
use futures::prelude::*;



#[tokio::main]
async fn main() {
    let auth = "deug";
    let incident_info = PartialIncident {
        name: "bruh_name".to_string(),
        user: "bruh_user".to_string(),
        processid: 1190,
        remoteip: "bruh_remoteip".to_string(),
        cmdrun: "bruh_cmdrun".to_string()
    };

    let payload = FullIncident {
        hostname: "bruh_hostname".to_string(),
        incident: incident_info
    };

    //let blocking_task = tokio::task::spawn_blocking(|| {
        //incident_alert(auth, payload);
    //});
    incident_alert(auth, payload).await;
}
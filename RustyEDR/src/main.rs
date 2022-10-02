mod webserver_lib;

use std::fs::File;
use std::io::prelude::*;
use webserver_lib::incident_alert;


fn main() {
    let auth = "debug";
    let payload = "My name jeff";

    incident_alert(auth, payload);
}
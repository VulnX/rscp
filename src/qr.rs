use local_ip_address;
use qrcode::{QrCode, render::unicode::Dense1x2};

use crate::server;

fn get_endpoint(action: server::Action) -> &'static str {
    match action {
        server::Action::Upload { recv_path: _ } => {
            "upload"
        },
        server::Action::Download { file_path: _ } => {
            "download"
        }
    }
}

pub fn show_qr(action: server::Action, port: u16) {
    let ip: Option<String> = match local_ip_address::local_ip() {
        Ok(ip) => Some(ip.to_string()),
        Err(local_ip_address::Error::LocalIpAddressNotFound) => None,
        Err(e) => panic!("{}", e)
    };
    
    if let Some(ip) = ip {
        let endpoint = get_endpoint(action);
        let url = format!("http://{}:{}/{}", ip, port, endpoint);
        let qr_code = QrCode::new(&url).unwrap();
        let qrcode_str = qr_code.render::<Dense1x2>().quiet_zone(false).module_dimensions(1, 1).build();
        println!("Scan the qrcode given below \n\n{}\n\nOr manually open the URL in browser {}", qrcode_str, url);
    } else {
        eprintln!("Failed to resolve local IP. Please do it yourself.");
    }
}
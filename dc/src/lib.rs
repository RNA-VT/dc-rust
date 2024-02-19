use core::str;
use esp_idf_svc::http::{Method};
use esp_idf_svc::http::server::{EspHttpServer};
use esp_idf_svc::io::{Write};
use esp_idf_svc;
use serde::{Serialize};

#[derive(Serialize)]
struct SpecificationParameter {
    name: String,
    #[serde(rename = "type")]
    param_type: String,
}

#[derive(Serialize)]
struct SpecificationEndpoint {
    method: String,
    parameters: Vec<SpecificationParameter>,
}

#[derive(Serialize)]
struct Specification {
    id: String,
    device_type: String,
    mac: String,
    endpoints: Vec<SpecificationEndpoint>,
}

pub fn server(mut server:EspHttpServer) -> Option<EspHttpServer> {
// Set the HTTP server
    // http://<sta ip>/ handler
    server.fn_handler("/", Method::Get, |request| {
        let html = index_html();
        let mut response = request.into_ok_response()?;
        response.write_all(html.as_bytes())?;
        Ok(())
    }).expect("Failed to Create Root Handler");

    server.fn_handler("/specification", Method::Post, |request|{
        // Buffer to store the MAC address
        let mut mac_addr: [u8; 6] = [0; 6];
        unsafe {
            esp_idf_svc::sys::esp_wifi_get_mac(esp_idf_svc::sys::wifi_interface_t_WIFI_IF_STA, mac_addr.as_mut_ptr()).to_string();
        }
        let mac_hex_strings: Vec<String> = mac_addr.iter().map(|b| format!("{:02X}", b)).collect();

        let specification = Specification {
            id: String::from("relay-") + &mac_hex_strings.join("-"),
            device_type: String::from("relay"),
            mac: mac_hex_strings.join(":"),
            endpoints: vec![
                SpecificationEndpoint {
                    method: String::from("open"),
                    parameters: vec![],
                },
                SpecificationEndpoint {
                    method: String::from("close"),
                    parameters: vec![],
                },
                // Add more endpoints as needed
            ],
        };

        let json_response = serde_json::to_string(&specification).expect("Failed to Serialize JSON");

        let mut response = request.into_ok_response().expect("Failed to create response from request");
        response.write(json_response.as_bytes()).expect("Failed to write JSON Response");
        Ok(())
    }).expect("Failed to create /specification handler.");

    server.fn_handler("/health", Method::Post, |_request| {
        Ok(())
    }).expect("Failed to create /health handler.");
    Some(server)
}


fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

fn index_html() -> String {
    templated("relay")
}

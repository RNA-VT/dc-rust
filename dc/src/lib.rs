use core::str;
use esp_idf_svc::http::{Method,};
use esp_idf_svc::http::server::{EspHttpServer};
use esp_idf_svc;
use serde::{Serialize};
use esp_idf_svc::io::Write;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::io::EspIOError;

#[derive(Serialize)]
pub struct SpecificationParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
}

#[derive(Serialize)]
pub struct SpecificationEndpoint {
    pub method: String,
    pub parameters: Vec<SpecificationParameter>,
}

#[derive(Serialize)]
pub struct Specification {
    id: String,
    device_type: String,
    mac: String,
    endpoints: Vec<SpecificationEndpoint>,
}

pub fn server(mut server: EspHttpServer, device_type: String, endpoints: Vec<SpecificationEndpoint>) -> Result<EspHttpServer,EspError> {

    // Buffer to store the MAC address
    let mut mac_addr: [u8; 6] = [0; 6];
    unsafe {
        esp_idf_svc::sys::esp_wifi_get_mac(esp_idf_svc::sys::wifi_interface_t_WIFI_IF_STA, mac_addr.as_mut_ptr()).to_string();
    }

    let mac_hex_strings: Vec<String> = mac_addr.iter().map(|b| format!("{:02X}", b)).collect();

    let mut id = String::new();
    id.push_str(device_type.as_str());
    id.push_str("-");
    id.push_str(&mac_hex_strings.join("-"));

    let specification = Specification {
        id,
        device_type,
        mac: mac_hex_strings.join(":"),
        endpoints
    };

    // http://<sta ip>/ handler
    server.fn_handler::<EspIOError,_>(
        "/",
        Method::Get,
        |request| Ok({
            let html = index_html();
            request.into_ok_response()?.write_all(html.as_bytes())?;
        })
    )?;

    server.fn_handler::<EspIOError,_>(
        "/specification",
        Method::Post,
        move |request|{
            let json_response = serde_json::to_string(&specification).expect("Failed to Serialize JSON");

            let mut response = request.into_ok_response().expect("Failed to create response from request");
            response.write(json_response.as_bytes()).expect("Failed to write JSON Response");
            Ok(())
        }
    )?;

    server.fn_handler::<EspIOError,_>(
        "/health",
        Method::Post,
        |_request| {
            Ok(())
        }
    )?;

    Ok(server)
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

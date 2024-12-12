use base64::{engine::general_purpose, Engine as _};
/// Contains base64 string conversion functions.
pub fn base64_string_from_bytes(data: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(&data)
}

use regex::Regex;
use wasm_bindgen::prelude::*;
/// Validates URLs.
#[wasm_bindgen]
pub fn valid_url(url: String) -> bool {
    Regex::new(r"(^https?://)|(^www\.)")
        .unwrap()
        .is_match(&url)
}
/// Validates wifi login credential strings.
pub fn valid_wifi_string(string: String) -> bool {
    Regex::new(r"^WIFI:(T:(WPA|WEP|nopass);)?S:(.+);(P:(.+);)?(H:true;)?;$")
        .unwrap()
        .is_match(&string)
}
/// Validates wifi login credentials.
#[wasm_bindgen]
pub fn valid_wifi(ssid: String, password: String, encryption: String, hidden: bool) -> bool {
    valid_wifi_string(create_wifi_string(&ssid, &password, &encryption, hidden))
}
/// Validates that text is not a URL or a wifi login credential string.
#[wasm_bindgen]
pub fn valid_text(string: String) -> bool {
    !(valid_url(string.clone()) || valid_wifi_string(string.clone()))
}

/// Assembles a wifi login credential string.
pub fn create_wifi_string(ssid: &String, password: &String, encryption: &String, hidden: bool) -> String {
    // Prefix the string with "WIFI:".
    let mut data = String::from("WIFI:");
    // Add the encryption type to the string.
    data.push_str(&("T:".to_owned() + encryption + ";"));
    // Add the network's ssid to the string.
    data.push_str(&("S:".to_owned() + ssid       + ";"));
    // Add the network's password to the string.
    data.push_str(&("P:".to_owned() + password   + ";"));
    // If the network is hidden,
    if hidden {
        // Add that it is hidden to the string.
        data.push_str("H:true;");
    }
    // Add the final semi-colon.
    data.push(';');
    return data
}

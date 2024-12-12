use wasm_bindgen::prelude::*;

pub mod qr;
pub mod helpers;

#[wasm_bindgen]
pub fn text(text: String, width: u32, format: String) -> String {
    if !helpers::valid_text(text.clone()) {
        return "".to_string();
    }
    let qr_code = qr::from::text(String::from(text));
    if format == "text".to_string() {
        return qr_code.to_text();
    }
    else if format == "svg".to_string() {
        return qr_code.to_svg();
    }
    else if format == "png".to_string() {
        return qr_code.to_png(width);
    }
    return "".to_string();
}

#[wasm_bindgen]
pub fn url(url: String, width: u32, format: String) -> String {
    if !helpers::valid_url(url.clone()) {
        return "".to_string();
    }
    let qr_code = qr::from::url(String::from(url));
    if format == "text".to_string() {
        return qr_code.to_text();
    }
    else if format == "svg".to_string() {
        return qr_code.to_svg();
    }
    else if format == "png".to_string() {
        return qr_code.to_png(width);
    }
    return "".to_string();
}

#[wasm_bindgen]
pub fn wifi(ssid: String, password: String, encryption: String, hidden: bool, width: u32, format: String) -> String {
    if !helpers::valid_wifi(ssid.clone(), password.clone(), encryption.clone(), hidden) {
        return "".to_string();
    }
    let qr_code = qr::from::wifi(ssid, password, encryption, hidden);
    if format == "text".to_string() {
        return qr_code.to_text();
    }
    else if format == "svg".to_string() {
        return qr_code.to_svg();
    }
    else if format == "png".to_string() {
        return qr_code.to_png(width);
    }
    return "".to_string();
}

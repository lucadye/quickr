use fast_qr::qr::QRBuilder;
use fast_qr::QRCode;
use fast_qr::convert::{Builder, svg::SvgBuilder, image::ImageBuilder};

use crate::helpers;

/// Abstracts the generating of QR codes as text, SVGs, and PNGs.
pub struct QR {
    qr: QRCode,
}

impl QR {
    /// Generates a text-based QR code.
    pub fn to_text(&self) -> String {
        self.qr.to_str()
    }
    /// Generates an SVG-based QR code.
    pub fn to_svg(&self) -> String {
        // Generate the SVG
        SvgBuilder::default()
            // Remove margin
            .margin(0)
            // Convert it to a string
            .to_str(&self.qr)
    }
    /// Generates an PNG-based QR code.
    pub fn to_png(&self, width: u32) -> String {
        // Generate the PNG
        let image_bytes = ImageBuilder::default()
            // Set the width
            .fit_width(width)
            // Remove margin
            .margin(0)
            // Convert it to a Vec<u8>
            .to_bytes(&self.qr);
        // Encode it as a base64 string
        helpers::base64_string_from_bytes(image_bytes.unwrap())
    }
}



/// Encodes data as a QR struct.
pub fn encode(data: String) -> QR {
    // Build the QR code.
    let qr_code = QRBuilder::new(data).build();
    // Wrap the QR code in the helper struct.
    QR { qr: qr_code.unwrap() }
}

/// Abstracts the details of encoding a QR code.
pub mod from {
    use crate::helpers;
    /// Encodes a QR code of plaintext.
    pub fn text(text: String) -> super::QR {
        super::encode(text)
    }
    /// Encodes a QR code of a URL.
    pub fn url(url: String) -> super::QR {
        super::encode(url)
    }
    /// Encodes a QR code for logging into a wifi network.
    pub fn wifi(ssid: String, password: String, encryption: String, hidden: bool) -> super::QR {
        super::encode(helpers::create_wifi_string(&ssid, &password, &encryption, hidden))
    }

}

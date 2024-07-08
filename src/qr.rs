use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use image::{Luma, ImageBuffer};

pub fn qrgen(name: &str, phone: &str, email: &str) {
    // Create a vCard string
    let vcard = format!(
        "BEGIN:VCARD\nVERSION:3.0\nFN:{}\nTEL:{}\nEMAIL:{}\nEND:VCARD",
        name, phone, email
    );
    
    let qr = QrCode::encode_text(&vcard, QrCodeEcc::Medium).unwrap();

    // Create the image buffer
    let size = qr.size();
    let scale = 10; // scale factor for the QR code image
    let border = 4; // border size for the QR code image
    let img_size = (size + border * 2) * scale;
    let mut img = ImageBuffer::new(img_size as u32, img_size as u32);

    // Fill the image buffer with the QR code data
    for y in 0..img_size {
        for x in 0..img_size {
            let color = if qr.get_module(x / scale - border, y / scale - border) {
                Luma([0u8]) // Black
            } else {
                Luma([255u8]) // White
            };
            img.put_pixel(x as u32, y as u32, color);
        }
    }

    // Save the image as a PNG file
    img.save("contact_qr_code.png").unwrap();

    println!("QR code with contact information generated and saved as contact_qr_code.png");
}

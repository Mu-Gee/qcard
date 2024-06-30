use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
use image::{Luma, ImageBuffer};

pub fn qrgen() {
    // Contact information
    let name = "John Doe";
    let phone = "+1234567890";
    let email = "johndoe@example.com";

    // Create a vCard string
    let vcard = format!(
        "BEGIN:VCARD\nVERSION:3.0\nFN:{}\nTEL:{}\nEMAIL:{}\nEND:VCARD",
        name, phone, email
    );

    // Generate the QR code
    let qr = QrCode::encode_text(&vcard, QrCodeEcc::Medium).unwrap();

    // Create the image buffer
    let size = qr.size();
    let mut img = ImageBuffer::new(size as u32, size as u32);

    // Fill the image buffer with the QR code data
    for y in 0..size {
        for x in 0..size {
            let color = if qr.get_module(x, y) {
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

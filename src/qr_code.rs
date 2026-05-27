use godot::classes::image::Format;
use godot::classes::Image;
use godot::prelude::*;
use qrcode::{Color as QrColor, QrCode};

#[derive(GodotClass)]
#[class(base=RefCounted, no_init)]
pub struct WharfkitQrCode;

#[godot_api]
impl WharfkitQrCode {
    #[func]
    pub fn generate(data: GString, pixels_per_module: i32) -> Gd<Image> {
        let s = data.to_string();
        let ppm = pixels_per_module.max(1) as usize;
        let quiet_zone_px = 4 * ppm;
        let code = match QrCode::new(s.as_bytes()) {
            Ok(c) => c,
            Err(_) => return empty_image(),
        };
        let n = code.width();
        let pixels = (n * ppm) + 2 * quiet_zone_px;
        let mut bytes: Vec<u8> = vec![0xFF; pixels * pixels];
        let colors = code.to_colors();
        for my in 0..n {
            let y0 = quiet_zone_px + my * ppm;
            for mx in 0..n {
                if matches!(colors[my * n + mx], QrColor::Dark) {
                    let x0 = quiet_zone_px + mx * ppm;
                    for py in y0..(y0 + ppm) {
                        let row_start = py * pixels;
                        for px in x0..(x0 + ppm) {
                            bytes[row_start + px] = 0x00;
                        }
                    }
                }
            }
        }
        let data = PackedByteArray::from(bytes.as_slice());
        Image::create_from_data(
            pixels as i32,
            pixels as i32,
            false,
            Format::L8,
            &data,
        )
        .unwrap_or_else(empty_image)
    }
}

fn empty_image() -> Gd<Image> {
    Image::create_empty(1, 1, false, Format::L8).expect("1x1 image")
}

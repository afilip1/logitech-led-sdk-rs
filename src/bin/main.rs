extern crate logitech_led_sdk;
use logitech_led_sdk::*;

fn main() {
    if let Ok(app) = LogiLedApp::new() {
        println!("Initialized SDK");

        if let Ok((maj, min, build)) = app.get_sdk_version() {
            println!("Version: {}.{}.{}", maj, min, build);
    
            let mut bitmap = [0; 504];
            for (i, bytes) in bitmap.chunks_mut(4).enumerate() {
                bytes[0] = (7.0 * i as f64 / 126.0 * 255.0) as u8;
                bytes[1] = (255.0 - (i as f64 / 126.0 * 255.0)) as u8;
                bytes[3] = 255;
            }

            loop {
                app.set_lighting_from_bitmap(&bitmap).unwrap();
                bitmap.rotate_left(4);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}

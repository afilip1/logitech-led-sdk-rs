extern crate logitech_led_sdk;

use logitech_led_sdk::*;

fn main() {
    if let Ok(sdk) = LogiLedSdk::init() {
        println!("Initialized SDK");

        if let Ok(version) = sdk.get_sdk_version() {
            println!("{}", version);
        }

        // let mut bitmap = [0; 504];
        // for (i, bytes) in bitmap.chunks_mut(4).enumerate() {
        //     bytes[0] = (7.0 * i as f64 / 126.0 * 255.0) as u8;
        //     bytes[1] = (255.0 - (i as f64 / 126.0 * 255.0)) as u8;
        //     bytes[3] = 255;
        // }

        let mut bitmap = [0; 2016];
        for (i, bytes) in bitmap.chunks_mut(4).enumerate() {
            if i % 3 == 0 {
                bytes[2] = i as u8
            };
            bytes[3] = 255;
        }

        loop {
            sdk.set_lighting_from_bitmap(&bitmap).unwrap();
            bitmap.rotate_left(84);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    } else {
        println!("Failed to initialize SDK");
        std::process::exit(1);
    }
}

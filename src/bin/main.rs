extern crate logitech_led_sdk;
use logitech_led_sdk::*;

fn main() {
    unsafe {
        println!("Initialized: {}", LogiLedInit());

        let (mut maj, mut min, mut build) = (0, 0, 0);
        LogiLedGetSdkVersion(&mut maj, &mut min, &mut build);
        println!("Version: {}.{}.{}", maj, min, build);

        LogiLedSaveCurrentLighting();
        LogiLedSetLighting(0, 0, 0);

        let mut bitmap = [0; 504];
        for (i, bytes) in bitmap.chunks_mut(4).enumerate() {
            bytes[0] = (i as f64 / 126.0 * 255.0) as u8;
            bytes[1] = (255.0 - (i as f64 / 126.0 * 255.0)) as u8;
            bytes[3] = 255;
        }

        // LogiLedPulseSingleKey(LogiLed::KeyName::GLogo, 100, 0, 0, 0, 0, 0, 1000, true);

        // println!("{:?}", &bitmap[..]);
        println!("{}", LogiLedSetLightingFromBitmap(bitmap.as_ptr()));
        LogiLedFlashSingleKey(
            LogiLed::KeyName::Z,
            100,
            0,
            0,
            LOGI_LED_DURATION_INFINITE,
            300,
        );
        // LogiLedPulseSingleKey(LogiLed::KeyName::X, 0, 0, 0, 100, 100, 0, 500, true);
        // LogiLedSetLightingForKeyWithKeyName(LogiLed::KeyName::F, 100, 0, 100);
        loop {}

        // LogiLedStopEffects();
        // LogiLedRestoreLighting();
        // LogiLedShutdown();
    }
}

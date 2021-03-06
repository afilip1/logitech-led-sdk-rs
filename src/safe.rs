use raw;
use std::{
    fmt::{self, Display, Formatter},
    result,
    time::Duration,
};

macro_rules! try_unsafe {
    ($e:expr) => {
        if unsafe { $e } {
            Ok(())
        } else {
            Err(Error::Unknown)
        }
    };
}

type Result = result::Result<(), Error>;

#[derive(Debug)]
pub enum Error {
    Unknown,
    BitmapTooSmall,
}

fn percentage(x: u8) -> i32 {
    i32::from(x / 255 * 100)
}

#[derive(Default)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub build: i32,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.build)
    }
}

// storing a private unit value to force the use of new()
// maybe there's a better way?
pub struct LogiLedSdk(());

impl LogiLedSdk {
    pub fn init() -> result::Result<LogiLedSdk, Error> {
        if unsafe { raw::LogiLedInit() } {
            Ok(LogiLedSdk(()))
        } else {
            Err(Error::Unknown)
        }
    }

    pub fn get_sdk_version(&self) -> result::Result<Version, Error> {
        let mut v = Version::default();
        if unsafe { raw::LogiLedGetSdkVersion(&mut v.major, &mut v.minor, &mut v.build) } {
            Ok(v)
        } else {
            Err(Error::Unknown)
        }
    }

    pub fn save_current_lighting(&self) -> Result {
        try_unsafe!{ raw::LogiLedSaveCurrentLighting() }
    }

    pub fn set_lighting(&self, red: u8, green: u8, blue: u8) -> Result {
        try_unsafe!{ raw::LogiLedSetLighting(percentage(red), percentage(green), percentage(blue)) }
    }

    pub fn restore_lighting(&self) -> Result {
        try_unsafe!{ raw::LogiLedRestoreLighting() }
    }

    pub fn flash_lighting(
        &self,
        red: u8,
        green: u8,
        blue: u8,
        duration: Duration,
        interval: Duration,
    ) -> Result {
        try_unsafe!{
            raw::LogiLedFlashLighting(
                percentage(red),
                percentage(green),
                percentage(blue),
                duration.as_millis() as i32,
                interval.as_millis() as i32,
            )
        }
    }

    pub fn pulse_lighting(
        &self,
        red: u8,
        green: u8,
        blue: u8,
        duration: Duration,
        interval: Duration,
    ) -> Result {
        try_unsafe!{
            raw::LogiLedPulseLighting(
                percentage(red),
                percentage(green),
                percentage(blue),
                duration.as_millis() as i32,
                interval.as_millis() as i32,
            )
        }
    }

    pub fn stop_effects(&self) -> Result {
        try_unsafe!{ raw::LogiLedStopEffects() }
    }

    pub fn set_lighting_from_bitmap(&self, bitmap: &[u8]) -> Result {
        if bitmap.len() < raw::LOGI_LED_BITMAP_SIZE as usize {
            Err(Error::BitmapTooSmall)
        } else {
            try_unsafe!{ raw::LogiLedSetLightingFromBitmap(bitmap.as_ptr()) }
        }
    }
}

impl Drop for LogiLedSdk {
    fn drop(&mut self) {
        unsafe {
            raw::LogiLedShutdown();
        }
    }
}

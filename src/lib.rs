mod raw;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InitError,
    SdkVersionError,
    LightingFromBitmapError,
}

pub struct LogiLedApp;

impl LogiLedApp {
    pub fn new() -> Result<LogiLedApp> {
        if unsafe { raw::LogiLedInit() } {
            Ok(LogiLedApp)
        } else {
            Err(Error::InitError)
        }
    }

    pub fn get_sdk_version(&self) -> Result<(i32, i32, i32)> {
        let (mut maj, mut min, mut build) = (0, 0, 0);
        if unsafe { raw::LogiLedGetSdkVersion(&mut maj, &mut min, &mut build) } {
            Ok((maj, min, build))
        } else {
            Err(Error::SdkVersionError)
        }
    }

    pub fn set_lighting_from_bitmap(&self, bitmap: &[u8]) -> Result<()> {
        if unsafe { raw::LogiLedSetLightingFromBitmap(bitmap.as_ptr()) } {
            Ok(())
        } else {
            Err(Error::LightingFromBitmapError)
        }
    }
}

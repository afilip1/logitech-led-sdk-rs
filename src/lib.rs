#![allow(non_camel_case_types, non_snake_case)]

pub type wchar_t = u16;

pub const LOGI_LED_BITMAP_WIDTH: i32 = 21;
pub const LOGI_LED_BITMAP_HEIGHT: i32 = 6;
pub const LOGI_LED_BITMAP_BYTES_PER_KEY: i32 = 4;

pub const LOGI_LED_BITMAP_SIZE: i32 =
    LOGI_LED_BITMAP_WIDTH * LOGI_LED_BITMAP_HEIGHT * LOGI_LED_BITMAP_BYTES_PER_KEY;

pub const LOGI_LED_DURATION_INFINITE: i32 = 0;

pub const LOGI_DEVICETYPE_MONOCHROME_ORD: i32 = 0;
pub const LOGI_DEVICETYPE_RGB_ORD: i32 = 1;
pub const LOGI_DEVICETYPE_PERKEY_RGB_ORD: i32 = 2;

pub const LOGI_DEVICETYPE_MONOCHROME: i32 = 1 << LOGI_DEVICETYPE_MONOCHROME_ORD;
pub const LOGI_DEVICETYPE_RGB: i32 = 1 << LOGI_DEVICETYPE_RGB_ORD;
pub const LOGI_DEVICETYPE_PERKEY_RGB: i32 = 2 << LOGI_DEVICETYPE_PERKEY_RGB_ORD;

pub const LOGI_DEVICETYPE_ALL: i32 =
    LOGI_DEVICETYPE_MONOCHROME | LOGI_DEVICETYPE_RGB | LOGI_DEVICETYPE_PERKEY_RGB;

pub mod LogiLed {
    #[repr(C)]
    pub enum KeyName {
        ESC = 0x01,
        F1 = 0x3b,
        F2 = 0x3c,
        F3 = 0x3d,
        F4 = 0x3e,
        F5 = 0x3f,
        F6 = 0x40,
        F7 = 0x41,
        F8 = 0x42,
        F9 = 0x43,
        F10 = 0x44,
        F11 = 0x57,
        F12 = 0x58,
        PRINT_SCREEN = 0x137,
        SCROLL_LOCK = 0x46,
        PAUSE_BREAK = 0x145,
        TILDE = 0x29,
        ONE = 0x02,
        TWO = 0x03,
        THREE = 0x04,
        FOUR = 0x05,
        FIVE = 0x06,
        SIX = 0x07,
        SEVEN = 0x08,
        EIGHT = 0x09,
        NINE = 0x0A,
        ZERO = 0x0B,
        MINUS = 0x0C,
        EQUALS = 0x0D,
        BACKSPACE = 0x0E,
        INSERT = 0x152,
        HOME = 0x147,
        PAGE_UP = 0x149,
        NUM_LOCK = 0x45,
        NUM_SLASH = 0x135,
        NUM_ASTERISK = 0x37,
        NUM_MINUS = 0x4A,
        TAB = 0x0F,
        Q = 0x10,
        W = 0x11,
        E = 0x12,
        R = 0x13,
        T = 0x14,
        Y = 0x15,
        U = 0x16,
        I = 0x17,
        O = 0x18,
        P = 0x19,
        OPEN_BRACKET = 0x1A,
        CLOSE_BRACKET = 0x1B,
        BACKSLASH = 0x2B,
        KEYBOARD_DELETE = 0x153,
        END = 0x14F,
        PAGE_DOWN = 0x151,
        NUM_SEVEN = 0x47,
        NUM_EIGHT = 0x48,
        NUM_NINE = 0x49,
        NUM_PLUS = 0x4E,
        CAPS_LOCK = 0x3A,
        A = 0x1E,
        S = 0x1F,
        D = 0x20,
        F = 0x21,
        G = 0x22,
        H = 0x23,
        J = 0x24,
        K = 0x25,
        L = 0x26,
        SEMICOLON = 0x27,
        APOSTROPHE = 0x28,
        ENTER = 0x1C,
        NUM_FOUR = 0x4B,
        NUM_FIVE = 0x4C,
        NUM_SIX = 0x4D,
        LEFT_SHIFT = 0x2A,
        Z = 0x2C,
        X = 0x2D,
        C = 0x2E,
        V = 0x2F,
        B = 0x30,
        N = 0x31,
        M = 0x32,
        COMMA = 0x33,
        PERIOD = 0x34,
        FORWARD_SLASH = 0x35,
        RIGHT_SHIFT = 0x36,
        ARROW_UP = 0x148,
        NUM_ONE = 0x4F,
        NUM_TWO = 0x50,
        NUM_THREE = 0x51,
        NUM_ENTER = 0x11C,
        LEFT_CONTROL = 0x1D,
        LEFT_WINDOWS = 0x15B,
        LEFT_ALT = 0x38,
        SPACE = 0x39,
        RIGHT_ALT = 0x138,
        RIGHT_WINDOWS = 0x15C,
        APPLICATION_SELECT = 0x15D,
        RIGHT_CONTROL = 0x11D,
        ARROW_LEFT = 0x14B,
        ARROW_DOWN = 0x150,
        ARROW_RIGHT = 0x14D,
        NUM_ZERO = 0x52,
        NUM_PERIOD = 0x53,
        G_1 = 0xFFF1,
        G_2 = 0xFFF2,
        G_3 = 0xFFF3,
        G_4 = 0xFFF4,
        G_5 = 0xFFF5,
        G_6 = 0xFFF6,
        G_7 = 0xFFF7,
        G_8 = 0xFFF8,
        G_9 = 0xFFF9,
        G_LOGO = 0xFFFF1,
        G_BADGE = 0xFFFF2,
    }

    #[repr(C)]
    pub enum DeviceType {
        Keyboard = 0x0,
        Mouse = 0x3,
        Mousemat = 0x4,
        Headset = 0x8,
        Speaker = 0xE,
    }
}

extern "C" {
    #[link_name = "?LogiLedInit@@YA_NXZ"]
    pub fn LogiLedInit() -> bool;

    #[link_name = "?LogiLedGetSdkVersion@@YA_NPEAH00@Z"]
    pub fn LogiLedGetSdkVersion(majorNum: *mut i32, minorNum: *mut i32, buildNum: *mut i32)
        -> bool;
    #[link_name = "?LogiLedGetConfigOptionNumber@@YA_NPEB_WPEAN@Z"]
    pub fn LogiLedGetConfigOptionNumber(configPath: *const wchar_t, defaultValue: *mut f64) -> bool;
    
    #[link_name = "?LogiLedGetConfigOptionBool@@YA_NPEB_WPEA_N@Z"]
    pub fn LogiLedGetConfigOptionBool(configPath: *const wchar_t, defaultValue: *mut bool) -> bool;
    
    #[link_name = "?LogiLedGetConfigOptionColor@@YA_NPEB_WPEAH11@Z"]
    pub fn LogiLedGetConfigOptionColor(
        configPath: *const wchar_t,
        defaultRed: *mut i32,
        defaultGreen: *mut i32,
        defaultBlue: *mut i32,
    ) -> bool;
    
    #[link_name = "?LogiLedGetConfigOptionRect@@YA_NPEB_WPEAH111@Z"]
    pub fn LogiLedGetConfigOptionRect(
        configPath: *const wchar_t,
        defaultX: *mut i32,
        defaultY: *mut i32,
        defaultWidth: *mut i32,
        defaultHeight: *mut i32,
    ) -> bool;
    
    // this function isn't exported by the static library???
    // #[link_name = ""]
    // fn LogiLedGetConfigOptionString(
    //     configPath: *const wchar_t,
    //     defaultValue: *mut wchar_t,
    //     bufferSize: i32,
    // ) -> bool;
    
    #[link_name = "?LogiLedGetConfigOptionKeyInput@@YA_NPEB_WPEA_WH@Z"]
    pub fn LogiLedGetConfigOptionKeyInput(
        configPath: *const wchar_t,
        defaultValue: *mut wchar_t,
        bufferSize: i32,
    ) -> bool;
    
    #[link_name = "?LogiLedGetConfigOptionSelect@@YA_NPEB_WPEA_WPEAH0H@Z"]
    pub fn LogiLedGetConfigOptionSelect(
        configPath: *const wchar_t,
        defaultValue: *mut wchar_t,
        valueSize: *mut i32,
        values: *const wchar_t,
        bufferSize: i32,
    ) -> bool;
    
    #[link_name = "?LogiLedGetConfigOptionRange@@YA_NPEB_WPEAHHH@Z"]
    pub fn LogiLedGetConfigOptionRange(
        configPath: *const wchar_t,
        defaultValue: *mut i32,
        min: i32,
        max: i32,
    ) -> bool;
    
    #[link_name = "?LogiLedSetConfigOptionLabel@@YA_NPEB_WPEA_W@Z"]
    pub fn LogiLedSetConfigOptionLabel(configPath: *const wchar_t, label: *mut wchar_t) -> bool;

    //Generic functions => Apply to any device type.
    #[link_name = "?LogiLedSetTargetDevice@@YA_NH@Z"]
    pub fn LogiLedSetTargetDevice(targetDevice: i32) -> bool;

    #[link_name = "?LogiLedSaveCurrentLighting@@YA_NXZ"]
    pub fn LogiLedSaveCurrentLighting() -> bool;

    #[link_name = "?LogiLedSetLighting@@YA_NHHH@Z"]
    pub fn LogiLedSetLighting(
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
    ) -> bool;

    #[link_name = "?LogiLedRestoreLighting@@YA_NXZ"]
    pub fn LogiLedRestoreLighting() -> bool;

    #[link_name = "?LogiLedFlashLighting@@YA_NHHHHH@Z"]
    pub fn LogiLedFlashLighting(
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
        milliSecondsDuration: i32,
        milliSecondsInterval: i32,
    ) -> bool;

    #[link_name = "?LogiLedPulseLighting@@YA_NHHHHH@Z"]
    pub fn LogiLedPulseLighting(
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
        milliSecondsDuration: i32,
        milliSecondsInterval: i32,
    ) -> bool;

    #[link_name = "?LogiLedStopEffects@@YA_NXZ"]
    pub fn LogiLedStopEffects();

    //Per-key functions => only apply to LOGI_DEVICETYPE_PERKEY_RGB devices.
    #[link_name = "?LogiLedSetLightingFromBitmap@@YA_NQEAE@Z"]
    pub fn LogiLedSetLightingFromBitmap(bitmap: *const u8) -> bool;

    #[link_name = "?LogiLedSetLightingForKeyWithScanCode@@YA_NHHHH@Z"]
    pub fn LogiLedSetLightingForKeyWithScanCode(
        keyCode: i32,
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
    ) -> bool;

    #[link_name = "?LogiLedSetLightingForKeyWithHidCode@@YA_NHHHH@Z"]
    pub fn LogiLedSetLightingForKeyWithHidCode(
        keyCode: i32,
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
    ) -> bool;

    #[link_name = "?LogiLedSetLightingForKeyWithQuartzCode@@YA_NHHHH@Z"]
    pub fn LogiLedSetLightingForKeyWithQuartzCode(
        keyCode: i32,
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
    ) -> bool;

    #[link_name = "?LogiLedSetLightingForKeyWithKeyName@@YA_NW4KeyName@LogiLed@@HHH@Z"]
    pub fn LogiLedSetLightingForKeyWithKeyName(
        keyName: LogiLed::KeyName,
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
    ) -> bool;

    #[link_name = "?LogiLedSaveLightingForKey@@YA_NW4KeyName@LogiLed@@@Z"]
    pub fn LogiLedSaveLightingForKey(keyName: LogiLed::KeyName) -> bool;

    #[link_name = "?LogiLedRestoreLightingForKey@@YA_NW4KeyName@LogiLed@@@Z"]
    pub fn LogiLedRestoreLightingForKey(keyName: LogiLed::KeyName) -> bool;

    #[link_name = "?LogiLedExcludeKeysFromBitmap@@YA_NPEAW4KeyName@LogiLed@@H@Z"]
    pub fn LogiLedExcludeKeysFromBitmap(keyList: *const LogiLed::KeyName, listCount: i32);

    //Per-key effects => only apply to LOGI_DEVICETYPE_PERKEY_RGB devices.
    #[link_name = "?LogiLedFlashSingleKey@@YA_NW4KeyName@LogiLed@@HHHHH@Z"]
    pub fn LogiLedFlashSingleKey(
        keyName: LogiLed::KeyName,
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
        msDuration: i32,
        msInterval: i32,
    ) -> bool;

    #[link_name = "?LogiLedPulseSingleKey@@YA_NW4KeyName@LogiLed@@HHHHHHH_N@Z"]
    pub fn LogiLedPulseSingleKey(
        keyName: LogiLed::KeyName,
        startRedPercentage: i32,
        startGreenPercentage: i32,
        startBluePercentage: i32,
        finishRedPercentage: i32,
        finishGreenPercentage: i32,
        finishBluePercentage: i32,
        msDuration: i32,
        isInfinite: bool,
    ) -> bool;

    #[link_name = "?LogiLedStopEffectsOnKey@@YA_NW4KeyName@LogiLed@@@Z"]
    pub fn LogiLedStopEffectsOnKey(keyName: LogiLed::KeyName) -> bool;

    //Zonal functions => only apply to devices with zones.
    #[link_name = "?LogiLedSetLightingForTargetZone@@YA_NW4DeviceType@LogiLed@@HHHH@Z"]
    pub fn LogiLedSetLightingForTargetZone(
        deviceType: LogiLed::DeviceType,
        zone: i32,
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
    ) -> bool;

    #[link_name = "?LogiLedShutdown@@YAXXZ"]
    pub fn LogiLedShutdown();
}

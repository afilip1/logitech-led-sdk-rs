#![allow(dead_code)]

#[repr(C)]
enum KeyName {
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
    PrintScreen = 0x137,
    ScrollLock = 0x46,
    PauseBreak = 0x145,
    Tilde = 0x29,
    One = 0x02,
    Two = 0x03,
    Three = 0x04,
    Four = 0x05,
    Five = 0x06,
    Six = 0x07,
    Seven = 0x08,
    Eight = 0x09,
    Nine = 0x0A,
    Zero = 0x0B,
    Minus = 0x0C,
    Equals = 0x0D,
    Backspace = 0x0E,
    Insert = 0x152,
    Home = 0x147,
    PageUp = 0x149,
    NumLock = 0x45,
    NumSlash = 0x135,
    NumAsterisk = 0x37,
    NumMinus = 0x4A,
    Tab = 0x0F,
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
    OpenBracket = 0x1A,
    CloseBracket = 0x1B,
    Backslash = 0x2B,
    KeyboardDelete = 0x153,
    End = 0x14F,
    PageDown = 0x151,
    NumSeven = 0x47,
    NumEight = 0x48,
    NumNine = 0x49,
    NumPlus = 0x4E,
    CapsLock = 0x3A,
    A = 0x1E,
    S = 0x1F,
    D = 0x20,
    F = 0x21,
    G = 0x22,
    H = 0x23,
    J = 0x24,
    K = 0x25,
    L = 0x26,
    Semicolon = 0x27,
    Apostrophe = 0x28,
    Enter = 0x1C,
    NumFour = 0x4B,
    NumFive = 0x4C,
    NumSix = 0x4D,
    LeftShift = 0x2A,
    Z = 0x2C,
    X = 0x2D,
    C = 0x2E,
    V = 0x2F,
    B = 0x30,
    N = 0x31,
    M = 0x32,
    Comma = 0x33,
    Period = 0x34,
    ForwardSlash = 0x35,
    RightShift = 0x36,
    ArrowUp = 0x148,
    NumOne = 0x4F,
    NumTwo = 0x50,
    NumThree = 0x51,
    NumEnter = 0x11C,
    LeftControl = 0x1D,
    LeftWindows = 0x15B,
    LeftAlt = 0x38,
    Space = 0x39,
    RightAlt = 0x138,
    RightWindows = 0x15C,
    ApplicationSelect = 0x15D,
    RightControl = 0x11D,
    ArrowLeft = 0x14B,
    ArrowDown = 0x150,
    ArrowRight = 0x14D,
    NumZero = 0x52,
    NumPeriod = 0x53,
    G1 = 0xFFF1,
    G2 = 0xFFF2,
    G3 = 0xFFF3,
    G4 = 0xFFF4,
    G5 = 0xFFF5,
    G6 = 0xFFF6,
    G7 = 0xFFF7,
    G8 = 0xFFF8,
    G9 = 0xFFF9,
    GLogo = 0xFFFF1,
    GBadge = 0xFFFF2,
}

enum DeviceType {
    Keyboard = 0x0,
    Mouse = 0x3,
    Mousemat = 0x4,
    Headset = 0x8,
    Speaker = 0xE,
}

#[link(name = "LogitechLEDLib", kind = "static")]
extern "C" {
    #[link_name = "?LogiLedInit@@YA_NXZ"]
    fn LogiLedInit() -> bool;

    #[link_name = "?LogiLedGetSdkVersion@@YA_NPEAH00@Z"]
    fn LogiLedGetSdkVersion(majorNum: *mut i32, minorNum: *mut i32, buildNum: *mut i32) -> bool;

    //Generic functions => Apply to any device type.
    #[link_name = "?LogiLedSetTargetDevice@@YA_NH@Z"]
    fn LogiLedSetTargetDevice(targetDevice: i32) -> bool;

    #[link_name = "?LogiLedSaveCurrentLighting@@YA_NXZ"]
    fn LogiLedSaveCurrentLighting() -> bool;

    #[link_name = "?LogiLedSetLighting@@YA_NHHH@Z"]
    fn LogiLedSetLighting(redPercentage: i32, greenPercentage: i32, bluePercentage: i32) -> bool;

    #[link_name = "?LogiLedRestoreLighting@@YA_NXZ"]
    fn LogiLedRestoreLighting() -> bool;

    #[link_name = "?LogiLedFlashLighting@@YA_NHHHHH@Z"]
    fn LogiLedFlashLighting(
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
        milliSecondsDuration: i32,
        milliSecondsInterval: i32,
    ) -> bool;

    #[link_name = "?LogiLedPulseLighting@@YA_NHHHHH@Z"]
    fn LogiLedPulseLighting(
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
        milliSecondsDuration: i32,
        milliSecondsInterval: i32,
    ) -> bool;

    #[link_name = "?LogiLedStopEffects@@YA_NXZ"]
    fn LogiLedStopEffects();

    //Per-key functions => only apply to LOGI_DEVICETYPE_PERKEY_RGB devices.
    // bool LogiLedSetLightingFromBitmap(unsigned char bitmap[]);
    // bool LogiLedSetLightingForKeyWithScanCode(int keyCode, int redPercentage, int greenPercentage, int bluePercentage);
    // bool LogiLedSetLightingForKeyWithHidCode(int keyCode, int redPercentage, int greenPercentage, int bluePercentage);
    // bool LogiLedSetLightingForKeyWithQuartzCode(int keyCode, int redPercentage, int greenPercentage, int bluePercentage);
    // bool LogiLedSetLightingForKeyWithKeyName(LogiLed::KeyName keyName, int redPercentage, int greenPercentage, int bluePercentage);
    // bool LogiLedSaveLightingForKey(LogiLed::KeyName keyName);
    // bool LogiLedRestoreLightingForKey(LogiLed::KeyName keyName);
    // bool LogiLedExcludeKeysFromBitmap(LogiLed::KeyName *keyList, int listCount);

    //Per-key effects => only apply to LOGI_DEVICETYPE_PERKEY_RGB devices.
    #[link_name = "?LogiLedFlashSingleKey@@YA_NW4KeyName@LogiLed@@HHHHH@Z"]
    fn LogiLedFlashSingleKey(
        keyName: KeyName,
        redPercentage: i32,
        greenPercentage: i32,
        bluePercentage: i32,
        msDuration: i32,
        msInterval: i32,
    ) -> bool;

    #[link_name = "?LogiLedPulseSingleKey@@YA_NW4KeyName@LogiLed@@HHHHHHH_N@Z"]
    fn LogiLedPulseSingleKey(
        keyName: KeyName,
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
    fn LogiLedStopEffectsOnKey(keyName: KeyName) -> bool;

    #[link_name = "?LogiLedShutdown@@YAXXZ"]
    fn LogiLedShutdown();
}

fn main() {
    unsafe {
        println!("Initialized: {}", LogiLedInit());

        let (mut maj, mut min, mut build) = (0, 0, 0);
        println!(
            "Retrieved SDK version: {}",
            LogiLedGetSdkVersion(&mut maj, &mut min, &mut build)
        );
        println!("Version: {}.{}.{}", maj, min, build);

        LogiLedSaveCurrentLighting();
        LogiLedSetLighting(0, 0, 0);

        LogiLedFlashSingleKey(KeyName::Z, 100, 0, 0, 0, 300);
        LogiLedPulseSingleKey(KeyName::X, 0, 0, 0, 100, 100, 0, 500, true);
        loop {}

        // LogiLedStopEffects();
        // LogiLedRestoreLighting();
        // LogiLedShutdown();
    }
}

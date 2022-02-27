use winapi::um::winuser::{INPUT, INPUT_KEYBOARD, INPUT_u, KEYBDINPUT, SendInput};

use std::thread::sleep;
use std::time::Duration;

const KEYUP: u32 = 0x0002;
const KEYDW: u32 = 0x0;

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum WinapiKeys {
    a0key = 0x30,
    a1key = 0x31,
    a2key = 0x32,
    a3key = 0x33,
    a4key = 0x34,
    a5key = 0x35,
    a6key = 0x36,
    a7key = 0x37,
    a8key = 0x38,
    a9key = 0x39,
    Akey = 0x41,
    Bkey = 0x42,
    Ckey = 0x43,
    Dkey = 0x44,
    Ekey = 0x45,
    Fkey = 0x46,
    Gkey = 0x47,
    Hkey = 0x48,
    Ikey = 0x49,
    Jkey = 0x4A,
    Kkey = 0x4B,
    Lkey = 0x4C,
    Mkey = 0x4D,
    Nkey = 0x4E,
    Okey = 0x4F,
    Pkey = 0x50,
    Qkey = 0x51,
    Rkey = 0x52,
    Skey = 0x53,
    Tkey = 0x54,
    Ukey = 0x55,
    Vkey = 0x56,
    Wkey = 0x57,
    Xkey = 0x58,
    Ykey = 0x59,
    Zkey = 0x5A,
    Leftmousebutton = 0x01,
    Rightmousebutton = 0x02,
    X1mousebutton = 0x05,
    X2mousebutton = 0x06,
    BACKSPACEkey = 0x08,
    TABkey = 0x09,
    CLEARkey = 0x0C,
    ENTERkey = 0x0D,
    SHIFTkey = 0x10,
    CTRLkey = 0x11,
    ALTkey = 0x12,
    PAUSEkey = 0x13,
    CAPSLOCKkey = 0x14,
    IMEKanamode = 0x15,
    IMEOn = 0x16,
    IMEJunjamode = 0x17,
    IMEfinalmode = 0x18,
    IMEHanjamode = 0x19,
    IMEOff = 0x1A,
    ESCkey = 0x1B,
    IMEconvert = 0x1C,
    IMEnonconvert = 0x1D,
    IMEaccept = 0x1E,
    IMEmodechangerequest = 0x1F,
    SPACEBAR = 0x20,
    PAGEUPkey = 0x21,
    PAGEDOWNkey = 0x22,
    ENDkey = 0x23,
    HOMEkey = 0x24,
    LEFTARROWkey = 0x25,
    UPARROWkey = 0x26,
    RIGHTARROWkey = 0x27,
    DOWNARROWkey = 0x28,
    SELECTkey = 0x29,
    PRINTkey = 0x2A,
    EXECUTEkey = 0x2B,
    PRINTSCREENkey = 0x2C,
    INSkey = 0x2D,
    DELkey = 0x2E,
    HELPkey = 0x2F,
    ComputerSleepkey = 0x5F,
    Numerickeypad0key = 0x60,
    Numerickeypad1key = 0x61,
    Numerickeypad2key = 0x62,
    Numerickeypad3key = 0x63,
    Numerickeypad4key = 0x64,
    Numerickeypad5key = 0x65,
    Numerickeypad6key = 0x66,
    Numerickeypad7key = 0x67,
    Numerickeypad8key = 0x68,
    Numerickeypad9key = 0x69,
    Multiplykey = 0x6A,
    Addkey = 0x6B,
    Separatorkey = 0x6C,
    Subtractkey = 0x6D,
    Decimalkey = 0x6E,
    Dividekey = 0x6F,
    F1key = 0x70,
    F2key = 0x71,
    F3key = 0x72,
    F4key = 0x73,
    F5key = 0x74,
    F6key = 0x75,
    F7key = 0x76,
    F8key = 0x77,
    F9key = 0x78,
    F10key = 0x79,
    F11key = 0x7A,
    F12key = 0x7B,
    F13key = 0x7C,
    F14key = 0x7D,
    F15key = 0x7E,
    F16key = 0x7F,
    F17key = 0x80,
    F18key = 0x81,
    F19key = 0x82,
    F20key = 0x83,
    F21key = 0x84,
    F22key = 0x85,
    F23key = 0x86,
    F24key = 0x87,
    Slashkey = 0xBF
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum DxKeys {
    keyEscape = 1,
    key1 = 2,
    key2 = 3,
    key3 = 4,
    key4 = 5,
    key5 = 6,
    key6 = 7,
    key7 = 8,
    key8 = 9,
    key9 = 10,
    key0 = 11,
    keyMinus = 12,
    keyEquals = 13,
    keyBackspace = 14,
    keyTab = 15,
    keyQ = 16,
    keyW = 17,
    keyE = 18,
    keyR = 19,
    keyT = 20,
    keyY = 21,
    keyU = 22,
    keyI = 23,
    keyO = 24,
    keyP = 25,
    keyLeftBracket = 26,
    keyRightBracket = 27,
    keyEnter = 28,
    keyLeftControl = 29,
    keyA = 30,
    keyS = 31,
    keyD = 32,
    keyF = 33,
    keyG = 34,
    keyH = 35,
    keyJ = 36,
    keyK = 37,
    keyL = 38,
    keySemicolon= 39,
    keyApostrophe= 40,
    keyTilde = 41,
    keyLeftShift = 42,
    keyBackSlash = 43,
    keyZ = 44,
    keyX = 45,
    keyC = 46,
    keyV = 47,
    keyB = 48,
    keyN = 49,
    keyM = 50,
    keyComma = 51,
    keyPeriod = 52,
    keyForwardSlash = 53,
    keyRightShift = 54,
    keyNumpad = 55,
    keyLeftAlt = 56,
    keySpacebar = 57,
    keyCapsLock = 58,
    keyF1 = 59,
    keyF2 = 60,
    keyF3 = 61,
    keyF4 = 62,
    keyF5 = 63,
    keyF6 = 64,
    keyF7 = 65,
    keyF8 = 66,
    keyF9 = 67,
    keyF10 = 68,
    keyNumLock = 69,
    keyScrollLock = 70,
    keyNumpad7 = 71,
    keyNumpad8 = 72,
    keyNumpad9 = 73,
    keyNumpadMinus = 74,
    keyNumpad4 = 75,
    keyNumpad5 = 76,
    keyNumpad6 = 77,
    keyNumpadPlus = 78,
    keyNumpad1 = 79,
    keyNumpad2 = 80,
    keyNumpad3 = 81,
    keyNumpad0 = 82,
    keyNumpadDot = 83,
    keyF11 = 87,
    keyF12 = 88,
    keyNumpadEnter = 156,
    keyRightControl = 157,
    keyNumpadSlash = 181,
    keyRightAlt = 184,
    keyHome = 199,
    keyUpArrow = 200,
    keyPageUp = 201,
    keyLeftArrow = 203,
    keyRightArrow = 205,
    keyEnd = 207,
    keyDownArrow = 208,
    keyPageDown = 209,
    keyInsert = 210,
    keyDelete = 211,
    keyLeftMouseButton = 256,
    keyRightMouseButton = 257,
    keyMiddleMouseWheel = 258,
    keyMouseButton3 = 259,
    keyMouseButton4 = 260,
    keyMouseButton5 = 261,
    keyMouseButton6 = 262,
    keyMouseButton7 = 263,
    keyMouseWheelUp = 264,
    keyMouseWheelDown = 265,
}

pub fn send_key_event(vk: u16, flags: u32, sc: u16) {
    let mut input_u: INPUT_u = unsafe { std::mem::zeroed() };
    unsafe {
        *input_u.ki_mut() = KEYBDINPUT {
            wVk: vk,
            wScan: sc,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: 0
        };
    }

    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: input_u
    };
    unsafe {
        SendInput(1, &mut input, std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn key_press(vk: WinapiKeys, sc: DxKeys) {
    let delay = 50;
    let sc = sc as u16;
    let vk = vk as u16;

    send_key_event(vk, KEYDW, sc);
    sleep(Duration::from_millis(delay));
    send_key_event(vk, KEYUP, sc)
}

pub fn key_down(vk: WinapiKeys, sc: DxKeys) {
    let sc = sc as u16;
    let vk = vk as u16;

    send_key_event(vk as u16, KEYDW, sc)
}

pub fn key_up(vk: WinapiKeys, sc: DxKeys) {
    let sc = sc as u16;
    let vk = vk as u16;

    send_key_event(vk, KEYUP, sc)
}
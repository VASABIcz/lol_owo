use std::{time, thread};
use reqwest::blocking::{Client, Request, ClientBuilder};
use reqwest::Certificate;
use inputbot::{KeybdKey::*, MouseButton::*, *};

extern "win64" {
    pub fn GetForegroundWindow() -> i32;
    pub fn GetWindowTextA(hwnd: i32, str_ptr: *mut i8, size: i32);
}

unsafe fn ptr_to_string(ptr: & *mut i8, buf_size: u32) -> String {
    let mut buf: Vec<char> = Vec::new();

    for x in 0..buf_size {
        let n = *ptr.offset(x as isize) as u8;
        if n == 0 {
            return String::from_iter(buf);
        }
        let c = char::from(*ptr.offset(x as isize) as u8);
        buf.push(c);
    }
    String::new()
}

fn is_rito(client: &Client) -> bool {
    // extract minins spawn event
    match client.execute(client.get("https://127.0.0.1:2999/liveclientdata").build().unwrap()) {
        Ok(_) => true,
        Err(_) => false
    }
}

unsafe fn is_rito_full() -> bool {
    let w = GetForegroundWindow();
    let buf = [1_i8;500].as_mut_ptr();

    GetWindowTextA(w, buf, 500);
    let str = ptr_to_string(&buf, 100);
    if str == "League of Legends (TM) Client" {
        return true;
    }
    return false;
}

fn make_client() -> Client {
    ClientBuilder::new()
        .add_root_certificate(Certificate::from_pem(include_bytes!("rito.cer")).unwrap())
        .build().unwrap()
}

fn main() {
    let mut payload_sent = false;

    let client = make_client();
    thread::spawn( || handle_input_events());

    loop {
        if is_rito(&client) {
            println!("riot is running");
            println!("wait for game to load");
            thread::sleep(time::Duration::from_secs(60*2));
            loop {
                if !payload_sent {
                    if unsafe {is_rito_full()} {
                        println!("sending payload");
                        send_payload();
                        payload_sent = true;
                    }
                }
                else {
                    if !is_rito(&client) {
                        println!("game ended");
                        payload_sent = false;
                        break;
                    }
                }
            }
        }
    }
}

fn send_payload() {
    LShiftKey.press();
    EnterKey.press();
    EnterKey.release();
    LShiftKey.release();

    KeySequence(" OwO").send();

    LShiftKey.press();
    KeybdKey::from(191).press();
    KeybdKey::from(191).release();
    LShiftKey.release();
    EnterKey.press();
    EnterKey.release();
}
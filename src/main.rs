mod keyboard_api;

use std::fmt::Write;
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::Certificate;
use serde_json::{Value};
use crate::keyboard_api::{DxKeys, key_down, key_press, key_up, WinapiKeys};

extern "win64" {
    pub fn GetForegroundWindow() -> i32;
    pub fn GetWindowTextA(hwnd: i32, str_ptr: *mut i8, size: i32);
}

unsafe fn ptr_to_string(ptr: & *mut i8, buf_size: u32) -> String {
    let mut res = String::new();

    for x in 0..buf_size {
        let n = *ptr.offset(x as isize) as u8;
        if n == 0 {
            return res;
        }
        res.write_char(n as char);
    }
    res
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
    drop(buf);
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

fn is_loaded(client: &Client) -> bool {
    match client.execute(client.get("https://127.0.0.1:2999/liveclientdata/eventdata").build().unwrap()) {
        Ok(it) => {
            let data = it.text().unwrap();
            let v: Value = serde_json::from_str(&data).unwrap();
            let events = match v["Events"].as_array() {
                Some(v) => v.len(),
                None => 0
            };
            if events > 0 {
                return true
            }
            false
        },
        Err(_) => false
    }
}

fn main() {
    let mut payload_sent = false;

    let client = make_client();
    send_payload();
    loop {
        if is_rito(&client) {
            println!("riot is running");
            println!("wait for game to load");
            loop {
                if !payload_sent && is_loaded(&client) {
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
    key_press(WinapiKeys::ENTERkey, DxKeys::keyEnter); // ENTER

    key_press(WinapiKeys::Slashkey, DxKeys::keyForwardSlash);  // /
    key_press(WinapiKeys::Akey,DxKeys::keyA); // a
    key_press(WinapiKeys::Lkey,DxKeys::keyL); // l
    key_press(WinapiKeys::Lkey,DxKeys::keyL); // l

    key_press(WinapiKeys::SPACEBAR, DxKeys::keySpacebar); // ' '

    key_down(WinapiKeys::SHIFTkey, DxKeys::keyLeftShift);
    key_press(WinapiKeys::Okey, DxKeys::keyO); // o
    key_up(WinapiKeys::SHIFTkey, DxKeys::keyLeftShift);
    key_press(WinapiKeys::Wkey, DxKeys::keyW); // w
    key_down(WinapiKeys::SHIFTkey, DxKeys::keyLeftShift);
    key_press(WinapiKeys::Okey, DxKeys::keyO); // o
    key_up(WinapiKeys::SHIFTkey, DxKeys::keyLeftShift);

    key_down(WinapiKeys::SHIFTkey, DxKeys::keyLeftShift);
    key_press(WinapiKeys::Slashkey, DxKeys::keyForwardSlash);  // /
    key_up(WinapiKeys::SHIFTkey, DxKeys::keyLeftShift);
    key_press(WinapiKeys::ENTERkey, DxKeys::keyEnter);
}
use std::{thread, time};
use qr_code::QrCode;
use chrono::Local;

fn main() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        let t = Local::now();
        let s = format!("{}", t);
        let qr = QrCode::new(s.clone()).unwrap();
        println!("{}", qr.to_string(false, 3));
        println!("{}", s);
        thread::sleep(time::Duration::from_secs(1));
    }
}

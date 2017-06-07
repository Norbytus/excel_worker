#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate simple_excel_writer as excel;

extern crate beanstalkd;

mod excel_msg;
mod message;

use message::Message;
use beanstalkd::Beanstalkd;

use std::thread;
use std::time::Duration;

const MESSAGE: &'static str = r#"{"files":[{"file_name":"/tmp/test.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]},{"file_name":"/tmp/tes2t.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]},{"file_name":"/tmp/testsd.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]},{"file_name":"/tmp/tessdadt.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]}]}"#;

fn main() {

    let mut queue = Beanstalkd::connect("0.0.0.0", 11300).unwrap();

    let _ = queue.watch("excel").unwrap();

    loop {
        match queue.reserve() {
            Ok((id, message)) => {
                let msg: Message = serde_json::from_str(&message).unwrap();
                let xlsx_files = msg.get_files().unwrap();
                xlsx_files.to_xlsx();
            },
            Err(e) => {
                println!("{:?}", e);
            },
        }
        thread::sleep(Duration::new(5, 0));
    }

}

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate simple_excel_writer as excel;

extern crate beanstalkd;

extern crate clap;

mod excel_msg;
mod message;

use message::Message;
use beanstalkd::Beanstalkd;

use std::thread;
use std::time::Duration;

const MESSAGE: &'static str = r#"{"files":[{"file_name":"/tmp/test.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]},{"file_name":"/tmp/tes2t.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]},{"file_name":"/tmp/testsd.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]},{"file_name":"/tmp/tessdadt.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]}]}"#;

use clap::{Arg, App};

fn main() {

    let app = App::new("Excel queue worker")
        .version("0.1")
        .author("Alex")
        .arg(Arg::with_name("Delay")
             .short("d")
             .long("delay")
             .takes_value(true))
        .arg(Arg::with_name("ip")
             .short("i")
             .long("ip")
             .takes_value(true))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .takes_value(true))
        .get_matches();

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

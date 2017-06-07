#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate simple_excel_writer as excel;

extern crate beanstalkd;

extern crate clap;

extern crate daemonize;

mod excel_msg;
mod message;

use message::Message;
use beanstalkd::Beanstalkd;
use daemonize::Daemonize;

use std::thread;
use std::time::Duration;

use clap::{Arg, App};

fn main() {

    let app = App::new("Worker for queue(Beanstalkd) create xlsx files from message and puck in zip file")
        .version("0.1")
        .author("Alexsander Startcev(Norbytus), norbyt93@gmail.com")
        .arg(Arg::with_name("delay")
             .short("d")
             .long("delay")
             .default_value("3")
             .takes_value(true))
        .arg(Arg::with_name("ip")
             .short("i")
             .long("ip")
             .default_value("localhost")
             .takes_value(true))
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .default_value("11300")
             .takes_value(true))
        .arg(Arg::with_name("tube")
             .short("t")
             .long("tube")
             .default_value("excel")
             .takes_value(true))
        .arg(Arg::with_name("daemon")
             .long("daemon"))
        .get_matches();

    let delay = app.value_of("delay").unwrap().to_string();
    let ip = app.value_of("ip").unwrap().to_string();
    let port = app.value_of("port").unwrap().to_string();
    let tube = app.value_of("tube").unwrap().to_string();

    if app.is_present("daemon") {
    let daemonize = Daemonize::new()
        .pid_file("/tmp/excel.pid")
        .chown_pid_file(false)
        .working_directory("/tmp")
        .user("nobody")
        .group("daemon")
        .privileged_action(move || {
            init(&delay, &ip, &port, &tube);
            ()
        });

    match daemonize.start() {
        Ok(_) => {},
        Err(e) => println!("{:?}", e),
    };
    } else {
        init(&delay, &ip, &port, &tube);
    }


}

fn init(delay: &str, ip: &str, port: &str, tube: &str) {

    use std::str::FromStr;

    let mut queue = Beanstalkd::connect(ip, u16::from_str(port).unwrap())
        .unwrap();

    let _ = queue.watch(tube).unwrap();

    loop {
        match queue.reserve() {
            Ok((_, message)) => {
                let msg: Message = serde_json::from_str(&message).unwrap();
                let xlsx_files = msg.get_files().unwrap();
                xlsx_files.to_xlsx();
            },
            Err(e) => {
                println!("{:?}", e);
            },
        }
        thread::sleep(Duration::new(u64::from_str(delay).unwrap_or(3), 0));
    }

}

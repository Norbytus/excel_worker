use excel_msg::XlsxFiles;
use serde_json;

use std::fs::{File, OpenOptions};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    message: String,
    sleep: i32,
    trys: i32,
}

impl Message {
    pub fn get_files(&self) -> Result<XlsxFiles, &'static str> {
        let files = serde_json::from_str(&self.message);
        match files {
            Ok(xlsx_files) => Ok(xlsx_files),
            Err(e) => Err("Wrong json format"),
        }
    }

    pub fn is_from_file(&mut self) {

        let path: Result<FromFile, serde_json::error::Error> = serde_json::from_str(&self.message);

        let is_file = match path {
            Ok(path_to) => {

                let file = OpenOptions::new()
                    .read(true)
                    .open(path_to.from_file);
                match file {
                    Ok(mut f) => {
                        let mut data = String::new();
                        f.read_to_string(&mut data).unwrap();
                        Some(data)
                    },
                    Err(_) => None
                }

            },
            Err(_) => None
        };

        match is_file {
            Some(string) => self.message = string,
            None => {},
        }

    }
}

#[derive(Serialize, Deserialize, Debug)]
struct FromFile { from_file: String }

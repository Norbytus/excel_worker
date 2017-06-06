#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate simple_excel_writer as excel;

mod excel_msg;

use excel_msg::XlsxFiles;

const MESSAGE: &'static str = r#"{"files":[{"file_name":"/tmp/test.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]},{"file_name":"/tmp/tes2t.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]},{"file_name":"/tmp/testsd.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]},{"file_name":"/tmp/tessdadt.xlsx","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]}]}"#;

fn main() {

    let des: XlsxFiles = serde_json::from_str(MESSAGE).unwrap();
    des.to_xlsx();

}


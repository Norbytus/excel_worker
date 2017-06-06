#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate simple_excel_writer as excel;

use excel::*;

use serde_json::Value;

use std::thread;
use std::sync::Arc;

const MESSAGE: &'static str = r#"{"files":[{"file_name":"test","sheets":[{"sheet_name":"t","fields":[["string",32,false],[false,32,"string"]]}]}]}"#;

fn main() {

    // let t: serde_json::Value = serde_json::from_str(MESSAGE).unwrap();
    // println!("{:?}", t);
    let des: XlsxFiles = serde_json::from_str(MESSAGE).unwrap();
    des.to_xlsx();

}

#[derive(Serialize, Deserialize, Debug)]
struct XlsxFiles {
    files: Vec<XlsxFile>,
}

#[derive(Serialize, Deserialize, Debug)]
struct XlsxFile {
    file_name: String,
    sheets: Vec<Sheet>
}

#[derive(Serialize, Deserialize, Debug)]
struct Sheet {
    sheet_name: String,
    fields: Vec<Vec<Value>>,
}

impl XlsxFiles {

    fn to_xlsx(&self) {

        for file in &self.files {
            let arc_file = Arc::new(file);
            let file_clone = arc_file.clone();
            std::thread::spawn(move || {
                file_clone.save();
            });
        }

    }

}

impl XlsxFile {

    fn save(&self) {
        let mut workbook = Workbook::create(&self.file_name);
        for sheet in &self.sheets {
            sheet.add_sheet(&mut workbook);
        }
        workbook.close();
    }

}

impl Sheet {

    fn add_sheet(&self, wb: &mut Workbook) {

        let mut sheet = wb.create_sheet(&self.sheet_name);


        let mut rows: Vec<Row> = Vec::new();
        for field in &self.fields {
            rows.push(Sheet::get_row(&field));
        }

        wb.write_sheet(&mut sheet, |sheet_writer| {
            let sw = sheet_writer;
            for row in rows {
                sw.append_row(row);
            }
            sw.append_row(row![])
        }).expect("write excel error!");


    }

    fn get_row(row: &Vec<Value>) -> Row {

        let mut excel_row = Row::new();

        for column in row {
            match column {
                &Value::Null => excel_row.add_cell(""),
                &Value::Bool(b) => excel_row.add_cell(b),
                &Value::Number(ref n) => excel_row.add_cell(n.as_f64().unwrap_or(0.0)),
                &Value::String(ref text) => excel_row.add_cell(text.to_string()),
                _ => {},
            }
        }

        excel_row

    }

}

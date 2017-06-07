use excel::*;

use serde_json::Value;

// use std::thread;
// use std::sync::Arc;
// use std::sync::mpsc::channel;

use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct XlsxFiles {
    files: Vec<XlsxFile>,
    to: String,
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

    pub fn to_xlsx(self) {

        // let (tx, rx) = channel();

        // let mut size = self.files.capacity() - 1;

        for file in self.files {
            let name = file.save();
            let cmd = format!("zip -rv {} {}", self.to, name);
            Command::new("sh")
                .arg("-c")
                .arg(cmd)
                .spawn()
                .unwrap()
                .wait();
            let mut path = PathBuf::from(&name);
            path.pop();
            // let arc_file = Arc::new(file);
            // let file_clone = arc_file.clone();
            // let sender = tx.clone();
            // thread::spawn(move || {
            //     match sender.send(file_clone.save()) {
            //         Ok(_) => {},
            //         Err(e) => println!("{:?}", e),
            //     };
            // });
        }

        // while size != 0 {
        //     match rx.recv() {
        //         Ok(_) => size -= 1,
        //         Err(e) => {println!("{:?}", e);},
        //     };
        // }

    }

}

impl XlsxFile {

    fn save(&self) -> String {
        let mut workbook = Workbook::create(&self.file_name);
        for sheet in &self.sheets {
            sheet.add_sheet(&mut workbook);
        }
        workbook.close();
        self.file_name.to_string()
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

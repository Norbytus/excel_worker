use excel_msg::XlsxFiles;
use serde_json;

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
}

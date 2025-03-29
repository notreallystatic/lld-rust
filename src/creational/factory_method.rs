/*
   Problem statement: Document editor application. Lets keep it minimal.
       We will read and parse the data and store it in a struct. After that we will print the struct value as json.
       Support csv and json.
       Consider there is a single json object in case of json file. In case of csv, consider only the first row.
*/

use serde::{Deserialize, Serialize};
use std::{error::Error, io};

#[derive(Serialize, Deserialize, Debug)]
struct DocData {
    name: String,
    age: u16,
}

enum DocumentType {
    Json,
    Csv,
}

trait DocumentProcessor {
    fn read_data(&self, file_name: String) -> Result<DocData, Box<dyn Error>>;
}

struct CsvProcessor {}

impl DocumentProcessor for CsvProcessor {
    fn read_data(&self, file_name: String) -> Result<DocData, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_path(file_name)?;
        for result in rdr.deserialize() {
            let record: DocData = result?;
            return Ok(record);
        }
        Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "record not found",
        )))
    }
}

struct JsonProcessor {}

impl DocumentProcessor for JsonProcessor {
    fn read_data(&self, file_name: String) -> Result<DocData, Box<dyn Error>> {
        let file_data = std::fs::read_to_string(file_name)?;
        let data: DocData = serde_json::from_str(&file_data)?;
        Ok(data)
    }
}

struct DocumentEditor {
    file_name: String,
    reader: Box<dyn DocumentProcessor>,
}

impl DocumentEditor {
    fn read_data(&self) -> Result<DocData, Box<dyn Error>> {
        self.reader.read_data(self.file_name.clone())
    }
}

struct DocumentEditorFactory {}

impl DocumentEditorFactory {
    fn create_editor(file_name: String, doc_type: DocumentType) -> DocumentEditor {
        match doc_type {
            DocumentType::Csv => DocumentEditor {
                file_name,
                reader: Box::new(CsvProcessor {}),
            },
            DocumentType::Json => DocumentEditor {
                file_name,
                reader: Box::new(JsonProcessor {}),
            },
        }
    }
}

pub fn run() {
    let file_info_list: Vec<(&str, DocumentType)> = vec![
        ("data.json", DocumentType::Json),
        ("data.csv", DocumentType::Csv),
    ];
    for file_info in file_info_list {
        let document_editor =
            DocumentEditorFactory::create_editor(file_info.0.to_string(), file_info.1);
        let doc_data = document_editor
            .read_data()
            .expect("Error reading data from doc");
        println!("file :: {}, doc_data :: {:?}", file_info.0, doc_data);
    }
}

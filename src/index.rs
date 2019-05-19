use std::fmt;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

use serde_json::Value;

#[derive(Debug, PartialEq)]
enum ColumnType {
    Str,
    Number,
    Bool,
}

impl fmt::Display for ColumnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColumnType::Bool => write!(f, "{}", "bool"),
            ColumnType::Number => write!(f, "{}", "number"),
            ColumnType::Str => write!(f, "{}", "string"),
        }
    }
}

const INDEX_FILE_PATH: &str = "./";

#[derive(Debug)]
struct Index {
    column_name: String,
    content_type: ColumnType,
    file: File,
}

// TODO: first_value_id
impl Index {
    pub fn new(
        field_name: &str,
        content_type: ColumnType,
        first_value: Value,
    ) -> Result<Index, IndexError> {
        let column_name = format!("{}.{}", field_name, content_type);

        let file = Index::open_index_file(&column_name)?;

        let mut i = Index {
            column_name,
            content_type,
            file,
        };

        i.write(1, first_value)?;

        Ok(i)
    }

    pub fn open_index_file(column_name: &String) -> std::io::Result<File> {
        OpenOptions::new()
            .append(true)
            .create(true)
            .open(column_name)
    }

    // assume that value matches the content_type, because you would need to do that for each field
    // before you wrote an event
    pub fn write(&mut self, id: i32, value: Value) -> Result<(), IndexError> {
        //https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=234271ce006fb30a72f712c13cf31f19
        //https://doc.rust-lang.org/std/macro.writeln.html
        //https://doc.rust-lang.org/std/primitive.i64.html#method.to_be_bytes
        //https://doc.rust-lang.org/std/primitive.f64.html#method.to_bits

        writeln!(self.file, "{}:{}", id, value)?;

        Ok(())
    }

    pub fn get_reader(&mut self) -> Result<BufReader<File>, IndexError> {
        let mut f = OpenOptions::new().read(true).open(&self.column_name)?;
        let mut reader = BufReader::new(f);
        Ok(reader)
    }
}

// create new index
//   create new file
//   open it for appending? hold on to that file handle?
// insert
//   take an id, value, append it to file
// get reader
//   open file for reading, give it to the caller
//

#[derive(Debug)]
pub struct IndexError {
    details: String,
}

impl IndexError {
    fn new(msg: String) -> IndexError {
        IndexError { details: msg }
    }
}

impl From<std::io::Error> for IndexError {
    fn from(err: std::io::Error) -> Self {
        IndexError::new(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use serde_json::Value;

    #[test]
    fn create_index() {
        // TODO: Clean up after yourself
        // https://www.reddit.com/r/rust/comments/5s2u5d/testing_file_operations_best_practices/
        let i = Index::new(
            "column",
            ColumnType::Str,
            Value::String("teehee".to_string()),
        )
        .unwrap();
        assert_eq!(i.column_name, "column.string");

        let i2 = Index::new("column", ColumnType::Bool, Value::Bool(true)).unwrap();
        assert_eq!(i2.column_name, "column.bool");

        let i3 = Index::new("column", ColumnType::Number, json!(55)).unwrap();
        assert_eq!(i3.column_name, "column.number")
    }

    #[test]
    fn read_index() {
        // TODO: Clean up after yourself
        // https://www.reddit.com/r/rust/comments/5s2u5d/testing_file_operations_best_practices/
        let mut i = Index::new(
            "readcolumn",
            ColumnType::Str,
            Value::String("teehee".to_string()),
        )
        .unwrap();
        assert_eq!(i.column_name, "readcolumn.string");

        let mut r = i.get_reader().unwrap();
        let mut s = String::new();
        let bytes_read = r.read_line(&mut s);

        assert_eq!("1:\"teehee\"\n", s)
    }
}

use parquet::basic::LogicalType;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::Field;

use std::{collections::HashMap, convert::TryFrom, path::Path};

pub struct Data {
    pub types: HashMap<String, LogicalType>,
    pub data: HashMap<String, Vec<i64>>,
}

impl Data {
    pub fn new() -> Self {
        Data {
            types: HashMap::new(),
            data: HashMap::new(),
        }
    }
}

pub fn load<'a>(path: &Path) -> parquet::errors::Result<Data> {
    let reader = SerializedFileReader::try_from(path)?;

    let mut data = Data::new();

    let fields = reader.metadata().file_metadata().schema().get_fields();
    for field in fields {
        let info = field.get_basic_info();
        let name = info.name().to_string();
        data.types.insert(name, info.logical_type());
    }

    for row in reader.into_iter() {
        for (name, value) in row.get_column_iter() {
            data.data
                .entry(name.to_string())
                .and_modify(|e| e.push(value));
        }
    }

    Ok(data)
}

#[cfg(test)]
mod tests {
    use crate::load::*;
    use std::path::Path;

    #[test]
    fn test_load() {
        let path = Path::new("./test/data/example.parquet");
        if let Ok(data) = load(path) {
            for (name, value) in data.data.iter() {
                println!("{} : {:?}", name, value);
            }
            assert!(false)
        } else {
            assert!(false)
        }
    }
}

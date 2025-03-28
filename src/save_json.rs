use serde::Serialize;

use std::{
    fs::File,
    io::{BufWriter, Write},
};

use serde_json::to_string_pretty;

pub fn save_json_to_file<T>(json_data: &T, filename: &str) -> Result<(), std::io::Error>
where
    T: Serialize,
{
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    let json_string = to_string_pretty(json_data)?;
    writer.write_all(json_string.as_bytes())?;
    writer.flush()?;
    Ok(())
}

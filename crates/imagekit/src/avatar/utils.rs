use std::{fs::File, io::Read};

use anyhow::Result;

pub fn parse_font_data(path: &str) -> Result<Vec<u8>> {
    let mut f = File::open(path)?;
    let mut font_data = Vec::new();
    f.read_to_end(&mut font_data)?;

    Ok(font_data)
}

pub fn parse_optional_t<T>(s: &str) -> Result<Option<T>>
where
    T: std::str::FromStr,
{
    if s.is_empty() {
        return Ok(None);
    }

    let t = s.parse::<T>();

    if let Ok(t) = t {
        return Ok(Some(t));
    }

    Ok(None)
}

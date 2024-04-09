use anyhow::Result;

pub async fn get_exif(path: &str) -> Result<exif::Exif> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    exif.fields().for_each(|field| {
        println!("{:?}", field);
    });

    Ok(exif)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_parse_image_exif() {
        let path = "/Users/fengwei/Downloads/3DE1CFFF-C11D-4068-9D8F-3A9129BEB3CB_1_105_c.jpeg";

        let file = std::fs::File::open(path).unwrap();
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        let exif = exifreader.read_from_container(&mut bufreader).unwrap();

        let gps_field = exif.get_field(exif::Tag::GPSDateStamp, exif::In::THUMBNAIL);

        if let Some(field) = gps_field {
            println!("gps: {:?}", field);
        }

        let fields = exif
            .fields()
            .fold(serde_json::map::Map::new(), |mut acc, f| {
                // f.tag.
                acc.insert(
                    f.tag.to_string(),
                    f.display_value().with_unit(&exif).to_string().into(),
                );
                acc
            });

        println!("{}", serde_json::to_string_pretty(&fields).unwrap());

        assert!(fields.len() > 0);
    }
}

//! Color module that helps generating and operating on rgb colors
#[cfg(feature = "cli")]
use anyhow::{Error, Result};
#[cfg(feature = "openapi")]
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Deserialize))]
pub struct RgbColor(u8, u8, u8);

impl RgbColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn random() -> Self {
        use getrandom::getrandom;

        let mut random_data = [0u8; 3];
        getrandom(&mut random_data).unwrap();
        Self(random_data[0], random_data[1], random_data[2])
    }

    /// Calculate the contrast ratio between colors
    pub fn find_ratio(&self, other: &Self) -> f32 {
        let l1 = self.luminance() + 0.05;
        let l2 = other.luminance() + 0.05;

        if l1 > l2 {
            l1 / l2
        } else {
            l2 / l1
        }
    }

    pub fn to_rgba(&self, alpha: u8) -> image::Rgba<u8> {
        image::Rgba([self.0, self.1, self.2, alpha])
    }

    fn luminance(&self) -> f32 {
        let r = self.0 as f32 / 255.0;
        let g = self.1 as f32 / 255.0;
        let b = self.2 as f32 / 255.0;

        let r = if r <= 0.03928 {
            r / 12.92
        } else {
            ((r + 0.055) / 1.055).powf(2.4)
        };

        let g = if g <= 0.03928 {
            g / 12.92
        } else {
            ((g + 0.055) / 1.055).powf(2.4)
        };

        let b = if b <= 0.03928 {
            b / 12.92
        } else {
            ((b + 0.055) / 1.055).powf(2.4)
        };

        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
}

#[cfg(feature = "cli")]
impl std::fmt::Display for RgbColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.0, self.1, self.2)
    }
}

#[cfg(feature = "cli")]
impl std::str::FromStr for RgbColor {
    type Err = Error;

    fn from_str(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 {
            return Err(Error::msg("invalid hex color"));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;

        Ok(Self(r, g, b))
    }
}

impl Default for RgbColor {
    fn default() -> Self {
        Self(255, 255, 255)
    }
}

#[cfg(feature = "openapi")]
impl salvo::oapi::ToSchema for RgbColor {
    fn to_schema(
        _components: &mut salvo::oapi::Components,
    ) -> salvo::oapi::RefOr<salvo::oapi::schema::Schema> {
        use salvo::oapi::schema::{Array, KnownFormat, Object, SchemaFormat, SchemaType};
        use serde_json::json;

        Array::new(Object::with_type(SchemaType::Integer))
            // .items(u8::to_schema(components))
            .items(
                Object::new()
                    .schema_type(SchemaType::Integer)
                    .format(SchemaFormat::KnownFormat(KnownFormat::UInt8))
                    .default_value(json!(255)),
            )
            .min_items(3)
            .max_items(3)
            .into()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Deserialize))]
pub struct RandomizedColors(pub bool, pub bool);

impl Default for RandomizedColors {
    fn default() -> Self {
        Self(true, true)
    }
}

#[cfg(feature = "cli")]
impl std::fmt::Display for RandomizedColors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

#[cfg(feature = "cli")]
impl std::str::FromStr for RandomizedColors {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let colors = s.split(',').collect::<Vec<&str>>();

        if colors.len() == 1 {
            let color = colors[0].parse::<bool>()?;
            return Ok(Self(color, color));
        }

        let font = colors[0].parse::<bool>()?;
        let bg = colors[1].parse::<bool>()?;

        Ok(Self(font, bg))
    }
}

#[cfg(feature = "openapi")]
impl salvo::oapi::ToSchema for RandomizedColors {
    fn to_schema(
        components: &mut salvo::oapi::Components,
    ) -> salvo::oapi::RefOr<salvo::oapi::schema::Schema> {
        use salvo::oapi::schema::{Array, Object, SchemaType};

        Array::new(Object::with_type(SchemaType::Boolean))
            // .items(u8::to_schema(components))
            .items(bool::to_schema(components))
            .min_items(2)
            .max_items(2)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "cli")]
    fn test_invalid_hex() {
        use std::str::FromStr;

        assert!(RgbColor::from_str("#12345").is_err());
    }

    #[test]
    fn test_rgb_color_new() {
        let color = RgbColor::new(255, 255, 255);
        assert_eq!(color, RgbColor(255, 255, 255));
    }

    #[test]
    #[cfg(feature = "openapi")]
    fn test_rgb_color_to_schema() {
        use salvo::oapi::{Components, ToSchema};
        use serde_json::json;

        let mut components = Components::new();
        let schema = RgbColor::to_schema(&mut components);
        let schema = serde_json::to_value(schema).unwrap();

        let value = json!({
            "type": "array",
            "items": {
                "type": "integer",
                "format": "uint8",
                "default": 255
            },
            "minItems": 3,
            "maxItems": 3
        });

        assert_eq!(schema, value);
    }

    #[test]
    #[cfg(feature = "openapi")]
    fn test_randomized_colors_to_schema() {
        use salvo::oapi::{Components, ToSchema};
        use serde_json::json;

        let mut components = Components::new();
        let schema = RandomizedColors::to_schema(&mut components);
        let schema = serde_json::to_value(schema).unwrap();

        let value = json!({
            "type": "array",
            "items": {
                "type": "boolean"
            },
            "minItems": 2,
            "maxItems": 2
        });

        assert_eq!(schema, value);
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_rgb_color_display() {
        let color = RgbColor::new(255, 255, 255);
        assert_eq!(color.to_string(), "#FFFFFF");
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_rgb_color_from_str() {
        use std::str::FromStr;

        let color = RgbColor::from_str("#FFFFFF").unwrap();
        assert_eq!(color, RgbColor(255, 255, 255));
    }
}

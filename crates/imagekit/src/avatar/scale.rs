#[cfg(feature = "cli")]
use std::fmt;

#[cfg(feature = "openapi")]
use salvo::oapi::ToSchema;
#[cfg(feature = "openapi")]
use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "openapi", derive(Deserialize, ToSchema))]
pub struct Scale {
    /// Horizontal scale, in pixels.
    pub x: f32,
    /// Vertical scale, in pixels.
    pub y: f32,
}

impl Scale {
    pub fn uniform(s: f32) -> Self {
        Self { x: s, y: s }
    }
}

impl Into<rusttype::Scale> for Scale {
    fn into(self) -> rusttype::Scale {
        rusttype::Scale {
            x: self.x,
            y: self.y,
        }
    }
}

#[cfg(feature = "cli")]
impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[cfg(feature = "cli")]
impl std::str::FromStr for Scale {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scales = s.split(',').collect::<Vec<&str>>();

        if scales.len() == 1 {
            let scale = scales[0].parse::<f32>()?;
            return Ok(Self::uniform(scale));
        }

        let x = scales[0].parse::<f32>()?;
        let y = scales[1].parse::<f32>()?;

        Ok(Self { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "cli")]
    fn test_scale_from_str() {
        use std::str::FromStr;

        assert_eq!(Scale::from_str("150").unwrap(), Scale::uniform(150.0));
        assert_eq!(Scale::from_str("150,150").unwrap(), Scale::uniform(150.0));

        assert_eq!(
            Scale::from_str("150,200").unwrap(),
            Scale { x: 150.0, y: 200.0 }
        );
    }

    #[test]
    #[cfg(feature = "cli")]
    fn test_scale_to_str() {
        assert_eq!(Scale::uniform(150.0).to_string(), "150,150");
        assert_eq!(Scale { x: 150.0, y: 200.0 }.to_string(), "150,200");
    }
}

use std::cmp;
#[cfg(feature = "cli")]
use std::str::FromStr;

use derive_builder::Builder;
use image::{imageops, DynamicImage, ImageBuffer, Rgba};
use rusttype::{point, Font};
#[cfg(feature = "openapi")]
use salvo::oapi::ToParameters;
#[cfg(feature = "openapi")]
use serde::Deserialize;

#[cfg(feature = "cli")]
use super::utils::{parse_font_data, parse_optional_t};
use super::{
    color::{RandomizedColors, RgbColor},
    scale::Scale,
};

/// Generate an initial avatar
#[derive(Builder)]
#[cfg_attr(feature = "openapi", builder(derive(ToParameters, Deserialize, Debug)), builder_struct_attr(salvo(parameters(default_parameter_in = Query))))]
#[cfg_attr(feature = "cli", derive(clap::Parser, Debug, Clone))]
pub struct Avatar {
    /// Initial name of the avatar
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<String>, required = true))))]
    #[cfg_attr(feature = "cli", clap(short, long = "name", value_name = "NAME"))]
    pub name: String,

    /// Vectorized font data
    #[builder(default = "None")]
    #[cfg_attr(
        feature = "cli",
        arg(
            short,
            long = "font",
            value_parser = parse_font_data,
            value_name = "FONT DATA"
        )
    )]
    font_data: Option<Vec<u8>>,

    /// Scale of the font
    #[builder(default = "Scale::uniform(150.0)")]
    #[cfg_attr(feature = "cli", clap(long, default_value_t = Scale::uniform(150.0), value_parser = Scale::from_str, value_name = "FONT SCALE"))]
    font_scale: Scale,

    /// RGB color of the font
    #[builder(default = "RgbColor::default()")]
    #[cfg_attr(feature = "cli", clap(long, default_value_t = RgbColor::default(), value_parser = RgbColor::from_str, value_name = "FONT COLOR"))]
    font_color: RgbColor,

    /// RGB color of the background
    #[builder(default = "RgbColor::new(224, 143, 112)")]
    #[cfg_attr(feature = "cli", clap(short, long, default_value_t = RgbColor::new(224, 143, 112), value_parser = RgbColor::from_str, value_name = "BACKGROUND COLOR"))]
    background_color: RgbColor,

    /// Size of the inner-text
    #[builder(default = "2")]
    #[cfg_attr(
        feature = "cli",
        clap(short, long, default_value_t = 2, value_name = "LENGTH")
    )]
    pub length: usize,

    /// Width of the avatar
    #[builder(default = "300")]
    #[cfg_attr(
        feature = "cli",
        clap(short, long, default_value_t = 300, value_name = "WIDTH")
    )]
    width: u32,

    /// Height of the avatar
    #[builder(default = "300")]
    #[cfg_attr(
        feature = "cli",
        clap(long, default_value_t = 300, value_name = "HEIGHT")
    )]
    height: u32,

    /// Contrast ratio for the colors
    #[builder(default = "4.5")]
    #[cfg_attr(
        feature = "cli",
        clap(long, default_value_t = 4.5, value_name = "CONTRAST RATIO")
    )]
    contrast_ratio: f32,

    /// Private property to hold if colors should be randomly generated
    #[builder(default = "RandomizedColors::default()")]
    #[cfg_attr(
        feature = "cli",
        clap(short, long, default_value_t = RandomizedColors::default(), value_parser = RandomizedColors::from_str, value_name = "RANDOMIZED COLORS")
    )]
    randomized_colors: RandomizedColors,

    /// Gaussian blur of the image
    #[builder(default = "None")]
    #[cfg_attr(
        feature = "cli",
        clap(long, value_parser = parse_optional_t::<f32>, value_name = "BLUR")
    )]
    blur: Option<f32>,
}

impl Avatar {
    pub fn draw(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let font_data = &self.font_data.to_owned().map_or(
            include_bytes!("fonts/Hiragino_Sans_GB_W3.ttf").to_vec(),
            |v| v,
        );

        // convert font-data vector to rusttype::Font
        let font = Font::try_from_bytes(&font_data).expect("Failed to parse font data");

        let scale: rusttype::Scale = self.font_scale.into();

        // substract metrics from the font according to the font scale
        let v_metrics = font.v_metrics(scale);

        // get the number of characters from the given name
        let text: String = self
            .name
            .chars()
            .take(cmp::min(self.length, self.name.len()))
            .collect();

        // layout the glyphs
        let glyphs: Vec<_> = font
            .layout(&text, scale, point(0.0, v_metrics.ascent))
            .collect();

        // substract height/width from the glyphs
        let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
        let glyphs_width = glyphs
            .iter()
            .rev()
            .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
            .next()
            .unwrap_or(0.0)
            .ceil() as u32;

        // calculate padding for glyphs
        let left_padding = (self.width - glyphs_width) / 2;
        let top_padding = (self.height - glyphs_height) / 2;

        // create dynamic RGBA image
        let mut image = DynamicImage::new_rgba8(self.width, self.height).to_rgba8();

        // randomize colors if not being settled
        let mut colors = self.randomized_colors;
        let mut background_color = self.background_color;
        let mut font_color = self.font_color;
        loop {
            if !colors.0 && !colors.1 {
                break;
            }

            if colors.0 | colors.1 {
                break;
            }

            if colors.0 {
                font_color = RgbColor::random();
            }

            if colors.1 {
                background_color = RgbColor::random();
            }

            colors = match font_color.find_ratio(&background_color) {
                // match if contrast ratio between colors is as expected
                r if r > self.contrast_ratio || r < 1. / self.contrast_ratio => {
                    RandomizedColors(false, false)
                }
                _ => {
                    if colors.0 | colors.1 {
                        colors
                    } else {
                        RandomizedColors(false, true)
                    }
                }
            }
        }

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // draw the glyph into the image according to font color
                glyph.draw(|x, y, v| {
                    image.put_pixel(
                        x + bounding_box.min.x as u32 + left_padding,
                        y + bounding_box.min.y as u32 + top_padding,
                        font_color.to_rgba((v * 255.0) as u8),
                    )
                });
            }
        }

        for (_, _, pixel) in image.enumerate_pixels_mut() {
            // put background pixels for the uncovered alpha channels
            if pixel[3] == 0 {
                *pixel = background_color.to_rgba(255)
            }
        }

        // apply gaussian blur to the image if specified
        if let Some(b) = self.blur {
            imageops::blur(&image, b)
        } else {
            image
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avatar_builder() {
        let avatar = AvatarBuilder::default()
            .name("test".to_string())
            .font_data(None)
            .font_color(RgbColor::new(0, 0, 0))
            .font_scale(Scale::uniform(120.0))
            .background_color(RgbColor::new(255, 255, 255))
            .length(10)
            .width(100)
            .height(100)
            .contrast_ratio(1.0)
            .randomized_colors(RandomizedColors(false, false))
            .blur(None)
            .build()
            .unwrap();

        assert_eq!(avatar.name, "test");
        assert!(avatar.font_data.is_none());
        assert_eq!(avatar.font_color, RgbColor::new(0, 0, 0));
        assert_eq!(avatar.font_scale, Scale::uniform(120.0));
        assert_eq!(avatar.background_color, RgbColor::new(255, 255, 255));
        assert_eq!(avatar.length, 10);
        assert_eq!(avatar.width, 100);
        assert_eq!(avatar.height, 100);
        assert_eq!(avatar.contrast_ratio, 1.0);
        assert_eq!(avatar.randomized_colors, RandomizedColors(false, false));
        assert_eq!(avatar.blur, None);
    }

    #[test]
    fn test_avatar_builder_partial_options() {
        let avatar = AvatarBuilder::default()
            .name("test".to_string())
            .build()
            .unwrap();

        assert_eq!(avatar.name, "test");
        assert_eq!(avatar.font_scale, Scale::uniform(150.0));
        assert_eq!(avatar.font_data.unwrap().len(), 13176596);
        assert_eq!(avatar.font_color, RgbColor::default());
        assert_eq!(avatar.background_color, RgbColor::new(224, 143, 112));
        assert_eq!(avatar.length, 2);
        assert_eq!(avatar.width, 300);
        assert_eq!(avatar.height, 300);
        assert_eq!(avatar.contrast_ratio, 4.5);
        assert_eq!(avatar.randomized_colors, RandomizedColors::default());
        assert_eq!(avatar.blur, None);
    }

    #[test]
    fn test_draw_avatar() {
        let avatar = AvatarBuilder::default()
            .name("test".to_string())
            .build()
            .unwrap();

        let image = avatar.draw();

        image.save("test.png").unwrap();
        assert_eq!(image.width(), 300);
        assert_eq!(image.height(), 300);
    }
}

mod utils;

use std::{
    fmt::Debug,
    io::{Cursor, Seek, Write},
    sync::Arc,
};

use anyhow::Result;
use clap::ValueEnum;
use derive_builder::Builder;
use image::{imageops::blur, ImageBuffer, ImageOutputFormat, Rgb};
#[cfg(feature = "openapi")]
use salvo::oapi::{ToParameters, ToSchema};
#[cfg(feature = "openapi")]
use serde::Deserialize;
use utils::*;

// value enum for the command line argument parser
#[derive(ValueEnum, Copy, Clone, Debug)]
#[cfg_attr(
    feature = "openapi",
    derive(Deserialize, ToSchema),
    serde(rename_all = "snake_case")
)]
pub enum ColorStyle {
    Bookworm,
    Jellyfish,
    Ten,
    Eleven,
    Mint,
    Greyscale,
    Christmas,
    Chameleon,
    Plasma,
    Plasma2,
    Config,
    Random,
}

impl Default for ColorStyle {
    fn default() -> Self {
        ColorStyle::Greyscale
    }
}

#[derive(Builder, Default)]
#[cfg_attr(feature = "openapi", builder(derive(Deserialize, ToParameters)), builder_struct_attr(salvo(parameters(default_parameter_in = Query))))]
pub struct Juliafatou {
    /// Image dimensions
    #[builder(setter(custom))]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<String>, default = "1200x1200"))))]
    dimensions: (usize, usize),

    /// Output file name
    #[builder(default = "String::from(\"output.png\")")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<String>, default = "output.png"))))]
    output_file: String,

    /// Offset for the viewpoint
    #[builder(setter(custom))]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<String>, default = "0.0,0.0"))))]
    offset: (f64, f64),

    /// Amount of blur to apply to the image
    #[builder(default = "3.0")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<f64>, default = "3.0"))))]
    scale: f64,

    /// Amount of blur to apply to the image
    #[builder(default = "1.0")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<f32>, default = "1.0"))))]
    blur: f32,

    /// Power for the second julia set, the 'x' in the equation z^x + c
    #[builder(default = "2")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<u8>, default = "2"))))]
    power: u8,

    /// Factor for the second julia set
    #[builder(default = "-0.25")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<f64>, default = "-0.25"))))]
    factor: f64,

    /// Color gradient style
    #[builder(default = "ColorStyle::Greyscale")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(inline, value_type = Option<ColorStyle>, default = "ColorStyle::Greyscale"))))]
    color_style: ColorStyle,

    /// Difference between the two rendered julia sets
    #[builder(default = "0.01")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<f64>, default = "0.01"))))]
    diverge: f64,

    /// The 'c' in the equation z^x + c
    #[builder(default = "String::from(\"-0.4,0.6\")")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<String>, default = "-0.4,0.6"))))]
    complex: String,

    /// Overall intensity multiplication factor
    #[builder(default = "3.0")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<f64>, default = "3.0"))))]
    intensity: f64,

    /// Whether to invert the colors
    #[builder(default = "false")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<bool>, default = "false"))))]
    inverse: bool,

    /// Number of threads to use
    #[builder(default = "None")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<usize>))))]
    threads: Option<usize>,

    /// Whether to print the time it took to render the image
    #[builder(default = "false")]
    #[cfg_attr(feature = "openapi", builder_field_attr(salvo(parameter(value_type = Option<bool>, default = "false"))))]
    take_time: bool,
}

impl JuliafatouBuilder {
    pub fn dimensions(&mut self, dimensions: String) -> &mut Self {
        let mut iterator = dimensions.split('x');

        let x = iterator
            .next()
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(1200);
        let y = iterator
            .next()
            .unwrap_or("0")
            .parse::<usize>()
            .unwrap_or(1200);

        self.dimensions = Some((x, y));

        self
    }

    pub fn offset(&mut self, offset: String) -> &mut Self {
        let mut iterator = offset.split(',');

        let x = iterator.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
        let y = iterator.next().unwrap_or("0").parse::<f64>().unwrap_or(0.0);

        self.offset = Some((x, y));

        self
    }
}

impl Juliafatou {
    fn partial_render(
        &self,
        pixels: &mut [u8],
        scale: (f64, f64),
        offset: (f64, f64, f64),
        bounds: (usize, usize),
        upper_left: (usize, usize),
        grad: &colorgrad::Gradient,
    ) {
        assert!(pixels.len() == bounds.0 * bounds.1 * 3);
        let power = self.power as u32;

        for column in 0..bounds.0 {
            for row in 0..bounds.1 {
                let x = row + upper_left.1;
                let y = column + upper_left.0;
                let point = (x, y);

                let diverged = get_diverged(parse_complex(&self.complex), self.diverge);

                let a = escape_time(point, scale, offset, diverged.0, 1024, power).unwrap_or(0.0);
                let b = escape_time(point, scale, offset, diverged.1, 1024, power).unwrap_or(0.0);

                let mut x = (a + b * self.factor) / (1.0 + self.factor);

                if self.inverse {
                    x = 255.0 - x;
                }

                let newpix: [u8; 4] = grad.reflect_at(x * self.intensity).to_rgba8();

                for rgb in 0..3 {
                    pixels[row * (bounds.0 * 3) + column * 3 + rgb] = newpix[rgb];
                }
            }
        }
    }

    fn get_pixels(&self) -> Result<Vec<u8>> {
        let color_array = return_colors(&self.color_style, None);

        // build color gradient
        let grad = colorgrad::CustomGradient::new()
            .colors(&color_array)
            .domain(&[0.0, 255.0])
            .mode(colorgrad::BlendMode::Rgb)
            .build()?;
        let grad_arc = Arc::new(grad);

        // initialize image buffer
        let mut pixels = vec![0u8; self.dimensions.0 * self.dimensions.1 * 3];
        // determine number of threads
        let threads = match self.threads {
            Some(value) => value,
            None => std::thread::available_parallelism()?.get(),
        };
        // determine maximum number of pixel rows per thread
        let rows_per_band = self.dimensions.1 / threads + 1;
        let scalex = self.scale / self.dimensions.1 as f64;

        // get x/y ratio of the image dimensions
        let ratio = self.dimensions.0 as f64 / self.dimensions.1 as f64;

        // calculate actual offset in a way that '0:0' will always result in a centered
        // image
        let off = self.scale / 2.0;
        let offset = ((self.offset.0 - off) + off * ratio, self.offset.1, off);

        {
            let bands: Vec<&mut [u8]> = pixels
                .chunks_mut(rows_per_band * self.dimensions.0 * 3)
                .collect();

            crossbeam::scope(|spawner| {
                for (i, band) in bands.into_iter().enumerate() {
                    let top = rows_per_band * i;

                    let height = band.len() / self.dimensions.0 / 3;

                    let band_upper_left = (0, top);

                    let band_bounds = (self.dimensions.0, height);

                    let cloned_arc = Arc::clone(&grad_arc);

                    spawner.spawn(move |_| {
                        self.partial_render(
                            band,
                            (scalex, scalex),
                            offset,
                            band_bounds,
                            band_upper_left,
                            &cloned_arc,
                        );
                    });
                }
            })
            .unwrap();
        }

        Ok(pixels)
    }

    fn blur_image<W>(&self, pixels: &[u8], buffered_writer: Option<&mut W>) -> Result<()>
    where
        W: Write + Seek,
    {
        if self.blur == 1.0 {
            if let Some(buffered_writer) = buffered_writer {
                buffered_writer.write_all(pixels)?;
            } else {
                image::save_buffer(
                    &self.output_file,
                    pixels,
                    self.dimensions.0 as u32,
                    self.dimensions.1 as u32,
                    image::ColorType::Rgb8,
                )?;
            }

            return Ok(());
        }

        let internalbuf = ImageBuffer::<Rgb<u8>, &[u8]>::from_raw(
            self.dimensions.0 as u32,
            self.dimensions.1 as u32,
            &pixels,
        )
        .unwrap();
        // .ok_or(anyhow!("error creating image buffer"))?;

        let blurred = blur(&internalbuf, self.blur);

        if let Some(mut buffered_writer) = buffered_writer {
            blurred.write_to(&mut buffered_writer, ImageOutputFormat::Png)?;
            return Ok(());
        }

        blurred.save(&self.output_file)?;

        Ok(())
    }

    // render the julia set to a buffer
    pub fn save_to_buffer<W>(&self, mut buffered_writer: W) -> Result<()>
    where
        W: Write + Seek,
    {
        let pixels = self.get_pixels()?;

        self.blur_image(&pixels, Some(&mut buffered_writer))?;

        Ok(())
    }

    // run the julia set renderer(save the image to a file)
    pub fn run(&self) -> Result<()> {
        let mut begin = None;
        if self.take_time {
            begin = Some(std::time::Instant::now());
        }

        println!("rendering julia set");

        let pixels = self.get_pixels()?;
        self.blur_image::<Cursor<Vec<u8>>>(&pixels, None)?;

        // take end time if flag has been set and print the duration
        if self.take_time {
            let duration = begin.unwrap().elapsed();
            eprintln!("time elapsed: {:?}", duration);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_builder() {
        let jf = JuliafatouBuilder::default().build().unwrap();

        assert_eq!(jf.dimensions, (1200, 1200));
    }

    #[test]
    fn test_builder_builder_with_values() {
        let jf = JuliafatouBuilder::default()
            .diverge(0.1)
            .offset(String::from("0.1,0.1"))
            .dimensions(String::from("1200x1200"))
            .build()
            .unwrap();

        assert_eq!(jf.dimensions, (1200, 1200));
        assert_eq!(jf.offset, (0.1, 0.1));
    }

    #[test]
    fn test_default_juliafatou_run() {
        let jf = JuliafatouBuilder::default()
            .take_time(true)
            .build()
            .unwrap();

        assert!(jf.run().is_ok());
    }

    #[test]
    fn test_juliafatou_save_to_buffer() {
        let jf = JuliafatouBuilder::default()
            .take_time(true)
            .build()
            .unwrap();

        let mut buffer = Cursor::new(Vec::new());
        assert!(jf.save_to_buffer(&mut buffer).is_ok());
    }
}

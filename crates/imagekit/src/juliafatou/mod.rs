use anyhow::Result;
use clap::ValueEnum;
use colorgrad::Color;
use derive_builder::Builder;
use getrandom::getrandom;
use image::imageops::blur;
use image::{ImageBuffer, Rgb};
use num::Complex;
use std::fs::read_to_string;
use std::sync::Arc;

// value enum for the command line argument parser

#[derive(ValueEnum, Copy, Clone, Debug)]
enum ColorStyle {
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

// calculate offset for viewpoint
fn calculate_offset(
    pixel: (usize, usize),
    scale: (f64, f64),
    offset: (f64, f64, f64),
) -> Complex<f64> {
    let cx = pixel.1 as f64 * scale.1 - (offset.2 + offset.0);
    let cy = pixel.0 as f64 * scale.0 - (offset.2 + offset.1);

    Complex { re: cx, im: cy }
}

// get smoothly shaded colors

fn make_smooth(c: Complex<f64>, i: usize) -> f64 {
    i as f64 + 2.0 - ((c.re.powi(2) + c.im.powi(2)).ln()).ln() / std::f64::consts::LN_2
}

// plotting algorithm for the julia set
fn escape_time(
    pixel: (usize, usize),
    scale: (f64, f64),
    offset: (f64, f64, f64),
    c: Complex<f64>,
    limit: usize,
    power: u32,
) -> Option<f64> {
    let mut z = calculate_offset(pixel, scale, offset);

    for i in 0..limit {
        if z.norm_sqr() > 5.0 {
            return Some(make_smooth(z, i));
        }

        z = z.powu(power) + c;
    }

    None
}

// calculate complex number for secundary julia set
fn get_diverged(c: Complex<f64>, diverge: f64) -> (Complex<f64>, Complex<f64>) {
    let altered = Complex {
        re: c.re + diverge,
        im: c.im - diverge,
    };

    (c, altered)
}

// return three colors for the color gradient
fn return_colors(style: &ColorStyle, path_opt: Option<String>) -> [Color; 3] {
    match style {
        ColorStyle::Bookworm => [
            Color::from_rgba8(5, 71, 92, 255),
            Color::from_rgba8(10, 120, 115, 255),
            Color::from_rgba8(184, 216, 215, 255),
        ],
        ColorStyle::Jellyfish => [
            Color::from_rgba8(38, 0, 24, 255),
            Color::from_rgba8(90, 25, 63, 255),
            Color::from_rgba8(198, 70, 72, 255),
        ],
        ColorStyle::Ten => [
            Color::from_rgba8(4, 62, 185, 255),
            Color::from_rgba8(2, 123, 230, 255),
            Color::from_rgba8(105, 254, 255, 255),
        ],
        ColorStyle::Greyscale => [
            Color::from_rgba8(255, 255, 255, 255),
            Color::from_rgba8(127, 127, 127, 255),
            Color::from_rgba8(0, 0, 0, 255),
        ],
        ColorStyle::Eleven => [
            Color::from_rgba8(2, 70, 217, 255),
            Color::from_rgba8(1, 214, 244, 255),
            Color::from_rgba8(209, 229, 254, 255),
        ],
        ColorStyle::Mint => [
            Color::from_rgba8(21, 21, 21, 255),
            Color::from_rgba8(137, 184, 70, 255),
            Color::from_rgba8(214, 214, 214, 255),
        ],
        ColorStyle::Chameleon => [
            Color::from_rgba8(11, 127, 109, 255),
            Color::from_rgba8(35, 145, 108, 255),
            Color::from_rgba8(21, 155, 110, 255),
        ],
        ColorStyle::Plasma => [
            Color::from_rgba8(35, 37, 83, 255),
            Color::from_rgba8(36, 102, 156, 255),
            Color::from_rgba8(219, 135, 75, 255),
        ],
        ColorStyle::Plasma2 => [
            Color::from_rgba8(0, 87, 139, 255),
            Color::from_rgba8(0, 147, 235, 255),
            Color::from_rgba8(249, 249, 249, 255),
        ],
        ColorStyle::Christmas => [
            Color::from_rgba8(31, 56, 35, 255),
            Color::from_rgba8(209, 27, 79, 255),
            Color::from_rgba8(250, 219, 82, 255),
        ],
        ColorStyle::Config => {
            get_colors_from_file(path_opt).expect("error parsing colors from file")
        }
        ColorStyle::Random => get_random_colors().expect("error getting random colors"),
    }
}

// get three colors from csv file - basic attempt

fn get_colors_from_file(path_opt: Option<String>) -> Result<[Color; 3]> {
    let filename = match path_opt {
        Some(path) => path,
        None => String::from("colors.csv"),
    };

    eprintln!("config file: '{}'", &filename);

    let mut output: [Color; 3] = [
        Color::from_rgba8(0, 0, 0, 0),
        Color::from_rgba8(0, 0, 0, 0),
        Color::from_rgba8(0, 0, 0, 0),
    ];

    let strings: Vec<String> = read_to_string(filename)?
        .lines()
        .skip(1)
        .map(String::from)
        .collect();

    assert!(strings.len() == 3);

    for string in strings.iter().enumerate() {
        let mut iterator = string.1.split(',');

        output[string.0] = Color::from_rgba8(
            iterator.next().unwrap_or("0").parse()?,
            iterator.next().unwrap_or("0").parse()?,
            iterator.next().unwrap_or("0").parse()?,
            255,
        );
    }

    Ok(output)
}

// get three random colors

fn get_random_colors() -> Result<[Color; 3]> {
    let mut random_data = [0u8; 9];

    getrandom(&mut random_data).map_err(anyhow::Error::msg)?;

    eprintln!(
        "R,G,B\n{},{},{}\n{},{},{}\n{},{},{}",
        random_data[0],
        random_data[1],
        random_data[2],
        random_data[3],
        random_data[4],
        random_data[5],
        random_data[6],
        random_data[7],
        random_data[8]
    );

    let output = [
        Color::from_rgba8(random_data[0], random_data[1], random_data[2], 255),
        Color::from_rgba8(random_data[3], random_data[4], random_data[5], 255),
        Color::from_rgba8(random_data[6], random_data[7], random_data[8], 255),
    ];

    Ok(output)
}

#[derive(Debug, Builder, Default)]
#[builder(setter(into))]
pub struct Juliafatou {
    /// Image dimensions
    #[builder(default = "(1200, 1200)")]
    dimensions: (usize, usize),

    /// Output file name
    #[builder(default = "String::from(\"output.png\")")]
    output_file: String,

    /// Offset for the viewpoint
    #[builder(default = "(0.0, 0.0)")]
    offset: (f64, f64),

    /// Amount of blur to apply to the image
    #[builder(default = "3.0")]
    scale: f64,

    /// Amount of blur to apply to the image
    #[builder(default = "1.0")]
    blur: f32,

    /// Power for the second julia set, the 'x' in the equation z^x + c
    #[builder(default = "2")]
    power: u8,

    /// Factor for the second julia set
    #[builder(default = "-0.25")]
    factor: f64,

    /// Color gradient style
    #[builder(default = "ColorStyle::Greyscale")]
    color_style: ColorStyle,

    /// Difference between the two rendered julia sets
    #[builder(default = "0.01")]
    diverge: f64,

    /// The 'c' in the equation z^x + c
    #[builder(default = "Complex { re: -0.4, im: 0.6 }")]
    complex: Complex<f64>,

    /// Overall intensity multiplication factor
    #[builder(default = "3.0")]
    intensity: f64,

    /// Whether to invert the colors
    #[builder(default = "false")]
    inverse: bool,

    /// Number of threads to use
    #[builder(default = "None")]
    threads: Option<usize>,

    /// Whether to print the time it took to render the image
    #[builder(default = "false")]
    take_time: bool,
}

impl Juliafatou {
    fn render(
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

                let diverged = get_diverged(self.complex, self.diverge);

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

    fn blur_image(&self, pixels: &[u8]) -> Result<()> {
        let internalbuf: ImageBuffer<Rgb<u8>, &[u8]> =
            ImageBuffer::from_raw(self.dimensions.0 as u32, self.dimensions.1 as u32, pixels)
                .unwrap();

        let blurred = blur(&internalbuf, self.blur);

        blurred.save(&self.output_file)?;

        Ok(())
    }

    pub fn run(&self) -> Result<()> {
        println!("rendering julia set");
        let mut begin = None;
        if self.take_time {
            begin = Some(std::time::Instant::now());
        }

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

        // calculate actual offset in a way that '0:0' will always result in a centered image
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
                        self.render(
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

        // minimalistic post processing
        self.blur_image(&pixels)
            .expect("error while blurring or writing the image");

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
            .dimensions((100, 100))
            .build()
            .unwrap();

        assert_eq!(jf.dimensions, (100, 100));
    }

    #[test]
    fn test_default_juliafatou_run() {
        let jf = JuliafatouBuilder::default()
            .take_time(true)
            .build()
            .unwrap();

        assert!(jf.run().is_ok());
    }
}

use anyhow::{anyhow, Result};
use clap::Parser;
use imagekit::{ColorStyle, JuliafatouBuilder};
use num::Complex;

#[derive(Parser, Debug, Clone)]
pub struct JuliafatouArgs {
    /// Image dimensions
    #[clap(
        short,
        long = "dimensions",
        default_value = "1200x1200",
        value_name = "USIZExUSIZE",
        value_parser = parse_dimensions
    )]
    dimensions: (usize, usize),

    /// Output file
    #[clap(
        short,
        long = "output-file",
        default_value = "output.png",
        value_name = "FILE"
    )]
    out: String,

    /// offset
    #[clap(
        short = 's',
        long = "offset",
        default_value = "0.0:0.0",
        allow_hyphen_values = true,
        value_name = "F64:F64",
        value_parser = parse_offset
    )]
    off: (f64, f64),

    /// scale factor
    #[clap(short = 'x', long = "scale", default_value_t = 3.0, value_name = "F64")]
    scale: f64,

    /// blur (sigma)
    #[clap(long, default_value_t = 1.0, value_name = "F32")]
    blur: f32,

    /// the 'x' in the equation z^x + c
    #[clap(short = 'w', long = "power", default_value_t = 2, value_name = "U8")]
    power: u8,

    /// multiplication factor of the secondary julia set (intensity)
    #[clap(short, long, default_value_t=-0.25, allow_hyphen_values=true, value_name="F64")]
    factor: f64,

    /// Select color gradient
    #[clap(
        short = 'c',
        long = "color-style",
        value_enum,
        default_value = "greyscale"
    )]
    cm: ColorStyle,

    /// difference between the two rendered julia sets
    #[clap(
        long = "diverge",
        value_name = "F64",
        default_value_t = 0.01,
        allow_hyphen_values = true
    )]
    diverge: f64,

    /// the 'c' in the equation z^x + c
    #[clap(
        short = 'p',
        long = "complex",
        value_name = "F64,F64",
        default_value = "-0.4,0.6",
        allow_hyphen_values = true,
        value_parser = parse_complex
    )]
    complex: Complex<f64>,

    /// overall intensity multiplication factor
    #[clap(short, long, value_name = "F64", default_value_t = 3.0)]
    intensity: f64,

    /// invert color gradient
    #[clap(long, default_value_t = false)]
    inverse: bool,

    /// number of threads (optional), defaults to 'available parallelism'
    #[clap(long, value_name = "USIZE")]
    threads: Option<usize>,

    /// measure render time
    #[clap(long, default_value_t = false)]
    take_time: bool,
}

pub async fn gen_julia_fatou(args: JuliafatouArgs) -> Result<()> {
    JuliafatouBuilder::default()
        .offset(args.off)
        .scale(args.scale)
        .blur(args.blur)
        .power(args.power)
        .factor(args.factor)
        .color_style(args.cm)
        .dimensions(args.dimensions)
        .diverge(args.diverge)
        .complex(args.complex)
        .intensity(args.intensity)
        .inverse(args.inverse)
        .take_time(args.take_time)
        .output_file(args.out)
        .threads(args.threads)
        .build()?
        .run()?;

    Ok(())
}

fn parse_offset(s: &str) -> Result<(f64, f64)> {
    let mut iter = s.split(':');
    let x = iter
        .next()
        .ok_or_else(|| anyhow!("Invalid offset"))?
        .parse()?;
    let y = iter
        .next()
        .ok_or_else(|| anyhow!("Invalid offset"))?
        .parse()?;

    Ok((x, y))
}

fn parse_dimensions(s: &str) -> Result<(usize, usize)> {
    let mut iter = s.split('x');
    let x = iter
        .next()
        .ok_or_else(|| anyhow!("Invalid dimensions"))?
        .parse()?;
    let y = iter
        .next()
        .ok_or_else(|| anyhow!("Invalid dimensions"))?
        .parse()?;

    Ok((x, y))
}

fn parse_complex(s: &str) -> Result<Complex<f64>> {
    let mut iter = s.split(',');
    let re = iter
        .next()
        .ok_or_else(|| anyhow!("Invalid complex"))?
        .parse()?;
    let im = iter
        .next()
        .ok_or_else(|| anyhow!("Invalid complex"))?
        .parse()?;

    Ok(Complex::new(re, im))
}
// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Blur {
        infile: String,
        outfile: String,
        blur_amount: Option<f32>,
    },
    Brighten {
        infile: String,
        outfile: String,
        brightness_amount: Option<i32>,
    },
    Rotate {
        infile: String,
        outfile: String,
        rotation_amount: u32,
    },
    Invert {
        infile: String,
        outfile: String,
    },
    Crop {
        infile: String,
        outfile: String,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
    },
    Grayscale {
        infile: String,
        outfile: String,
    },
    Generate {
        outfile: String,
        width: u32,
        height: u32,
        #[clap(arg_enum, value_parser)]
        stripe_orientation: StripeOrientation,
        colors: Vec<String>,
    },
    Fractal {
        outfile: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Blur {
            infile,
            outfile,
            blur_amount,
        } => {
            let blur_amount = if let Some(blur_amount) = blur_amount {
                blur_amount
            } else {
                2.0
            };
            blur(infile, outfile, blur_amount);
        }

        Commands::Brighten {
            infile,
            outfile,
            brightness_amount,
        } => {
            let brightness_amount = if let Some(brightness_amount) = brightness_amount {
                brightness_amount
            } else {
                10
            };
            brighten(infile, outfile, brightness_amount);
        }

        Commands::Rotate {
            infile,
            outfile,
            rotation_amount,
        } => {
            rotate(infile, outfile, rotation_amount);
        }

        Commands::Grayscale { infile, outfile } => {
            grayscale(infile, outfile);
        }

        Commands::Crop {
            infile,
            outfile,
            x,
            y,
            width,
            height,
        } => {
            crop(infile, outfile, x, y, width, height);
        }

        Commands::Generate {
            outfile,
            width,
            height,
            stripe_orientation,
            colors,
        } => {
            let colors = colors
                .iter()
                .map(|color_string| {
                    let split_vals = color_string.split(":");
                    let vec_vals = split_vals.collect::<Vec<&str>>();

                    Color {
                        red: vec_vals[0].parse::<u8>().unwrap(),
                        green: vec_vals[1].parse::<u8>().unwrap(),
                        blue: vec_vals[2].parse::<u8>().unwrap(),
                    }
                })
                .collect::<Vec<Color>>();

            generate(outfile, width, height, colors, stripe_orientation);
        }

        Commands::Invert { infile, outfile } => {
            invert(infile, outfile);
        }

        Commands::Fractal { outfile } => {
            fractal(outfile);
        }
    }
}

fn blur(infile: String, outfile: String, blur_amount: f32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    img.blur(blur_amount)
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

fn brighten(infile: String, outfile: String, brightness_amount: i32) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    img.brighten(brightness_amount)
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.crop(x, y, width, height)
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

fn rotate(infile: String, outfile: String, rotation_amount: u32) {
    let img = image::open(infile).expect("Failed to open INFILE.");

    match rotation_amount {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => panic!("{} is not a valid rotation amount!", rotation_amount),
    }
    .save(outfile)
    .expect("Failed writing OUTFILE.");
}

fn invert(infile: String, outfile: String) {
    let mut img = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed writing OUTFILE.");
}

fn grayscale(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    img.grayscale()
        .save(outfile)
        .expect("Failed writing OUTFILE.");
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum StripeOrientation {
    Vertical,
    Horizontal,
}

fn generate(
    outfile: String,
    width: u32,
    height: u32,
    colors: Vec<Color>,
    stripe_orientation: StripeOrientation,
) {
    let mut imgbuf = image::ImageBuffer::new(width, height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let color_index = match stripe_orientation {
            StripeOrientation::Vertical => (x as usize) / ((width as usize) / colors.len()),
            StripeOrientation::Horizontal => (y as usize) / ((height as usize) / colors.len()),
        };
        let curr_color = &colors[color_index];
        *pixel = image::Rgb([curr_color.red, curr_color.green, curr_color.blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!

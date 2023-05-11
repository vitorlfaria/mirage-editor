use crate::{
    commands::{blur, brighten, crop, fractal, generate, grayscale, invert, rotate},
    print_usage_and_exit,
};

pub fn match_one_command(subcommand: String, args: &mut Vec<String>) {
    match subcommand.as_str() {
        "blur" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile: String = args.remove(0);
            let outfile: String = args.remove(0);
            let amount_string: String = args.remove(0);
            let amount: f32 = amount_string.parse().expect("Failed to parse blur amount");
            blur(infile, outfile, amount);
        }

        "brighten" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }
            let infile: String = args.remove(0);
            let outfile: String = args.remove(0);
            let amount_string = args.remove(0);
            let amount: i32 = amount_string.parse().expect("Failed to parse blur amount");
            brighten(infile, outfile, amount);
        }

        "crop" => {
            if args.len() != 6 {
                print_usage_and_exit();
            }
            let infile: String = args.remove(0);
            let outfile: String = args.remove(0);

            let x_string: String = args.remove(0);
            let x: u32 = x_string.parse().expect("Failed to parse x value");

            let y_string: String = args.remove(0);
            let y: u32 = y_string.parse().expect("Failed to parse y value");

            let width_string: String = args.remove(0);
            let width: u32 = width_string.parse().expect("Failed to parse width value");

            let height_string: String = args.remove(0);
            let height: u32 = height_string.parse().expect("Failed to parse height value");

            crop(infile, outfile, x, y, width, height);
        }

        "rotate" => {
            if args.len() != 3 {
                print_usage_and_exit();
            }

            let infile: String = args.remove(0);
            let outfile: String = args.remove(0);
            let angle: String = args.remove(0);

            rotate(infile, outfile, angle);
        }

        "invert" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }

            let infile: String = args.remove(0);
            let outfile: String = args.remove(0);

            invert(infile, outfile);
        }

        "grayscale" => {
            if args.len() != 2 {
                print_usage_and_exit();
            }

            let infile: String = args.remove(0);
            let outfile: String = args.remove(0);

            grayscale(infile, outfile);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile: String = args.remove(0);
            fractal(outfile);
        }

        "generate" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile: String = args.remove(0);
            generate(outfile);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

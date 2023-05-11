use crate::commands::Commands;

pub fn apply_commands(commands: Commands, infile: String, outfile: String) {
    let img: image::DynamicImage = image::open(infile).expect("Failed to open INFILE.");
    let mut img2: image::DynamicImage = img;

    if commands.blur {
        img2 = img2.blur(commands.blur_amount);
    }

    if commands.brighten {
        img2 = img2.brighten(commands.brighten_amount);
    }

    if commands.rotate {
        img2 = match commands.angle.as_str() {
            "90" => img2.rotate90(),
            "180" => img2.rotate180(),
            "270" => img2.rotate270(),
            _ => img2,
        }
    }

    if commands.crop {
        img2 = img2.crop(
            commands.crop_x,
            commands.crop_y,
            commands.crop_width,
            commands.crop_height,
        );
    }

    if commands.invert {
        img2.invert();
    }

    if commands.grayscale {
        img2 = img2.grayscale();
    }

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

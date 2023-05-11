#[derive(Debug)]
pub struct Commands {
    pub blur: bool,
    pub blur_amount: f32,
    pub brighten: bool,
    pub brighten_amount: i32,
    pub crop: bool,
    pub crop_x: u32,
    pub crop_y: u32,
    pub crop_width: u32,
    pub crop_height: u32,
    pub rotate: bool,
    pub angle: String,
    pub invert: bool,
    pub grayscale: bool,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            blur: false,
            blur_amount: 5.0,
            brighten: false,
            brighten_amount: 10,
            crop: false,
            crop_x: 0,
            crop_y: 0,
            crop_width: 0,
            crop_height: 0,
            rotate: false,
            angle: String::from("90"),
            invert: false,
            grayscale: false,
        }
    }
}

impl Default for Commands {
    fn default() -> Self {
        Self::new()
    }
}

pub fn blur(infile: String, outfile: String, amount: f32) {
    let img: image::DynamicImage = image::open(infile).expect("Failed to open INFILE.");
    let img2: image::DynamicImage = img.blur(amount);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn brighten(infile: String, outfile: String, amount: i32) {
    let img: image::DynamicImage = image::open(infile).expect("Failed to open INFILE.");
    let img2: image::DynamicImage = img.brighten(amount);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {
    let mut img: image::DynamicImage = image::open(infile).expect("Failed to open INFILE.");
    let img2: image::DynamicImage = img.crop(x, y, width, height);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn rotate(infile: String, outfile: String, angle: String) {
    let img: image::DynamicImage = image::open(infile).expect("Failed to open INFILE.");

    let img2: image::DynamicImage = match angle.as_str() {
        "90" => img.rotate90(),
        "180" => img.rotate180(),
        "270" => img.rotate270(),
        _ => img.rotate270(),
    };

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn invert(infile: String, outfile: String) {
    let mut img: image::DynamicImage = image::open(infile).expect("Failed to open INFILE.");
    img.invert();
    img.save(outfile).expect("Failed to save OUTFILE.");
}

pub fn grayscale(infile: String, outfile: String) {
    let img: image::DynamicImage = image::open(infile).expect("Failed to open INFILE.");
    let img2: image::DynamicImage = img.grayscale();
    img2.save(outfile).expect("Failed to save OUTFILE.");
}

pub fn generate(outfile: String) {
    let width: u32 = 800;
    let height: u32 = 800;

    let mut img_buffer: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(width, height);

    let scale_x: f32 = 7.0 / width as f32;
    let scale_y: f32 = 9.0 / height as f32;

    for (x, y, pixel) in img_buffer.enumerate_pixels_mut() {
        let red: u8 = (0.4 * x as f32) as u8;
        let blue: u8 = (0.1 * y as f32) as u8;

        let cx: f32 = x as f32 * scale_x - 1.5;
        let cy: f32 = y as f32 * scale_y - 1.5;

        let c: num_complex::Complex<f32> = num_complex::Complex::new(-0.9, 0.1);
        let mut z: num_complex::Complex<f32> = num_complex::Complex::new(cx, cy);

        let mut green: u8 = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        *pixel = image::Rgb([red, green, blue]);
    }

    img_buffer.save(outfile).expect("Failed to OUTFILE.");
}

// This code was adapted from https://github.com/PistonDevelopers/image
pub fn fractal(outfile: String) {
    let width: u32 = 800;
    let height: u32 = 800;

    let mut imgbuf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(width, height);

    let scale_x: f32 = 3.0 / width as f32;
    let scale_y: f32 = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red: u8 = (0.3 * x as f32) as u8;
        let blue: u8 = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx: f32 = y as f32 * scale_x - 1.5;
        let cy: f32 = x as f32 * scale_y - 1.5;

        let c: num_complex::Complex<f32> = num_complex::Complex::new(-0.4, 0.6);
        let mut z: num_complex::Complex<f32> = num_complex::Complex::new(cx, cy);

        let mut green: u8 = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}

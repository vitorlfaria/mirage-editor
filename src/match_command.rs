use crate::commands::Commands;

pub fn match_command(args: &mut Vec<String>, commands: &mut Commands) {
    let command: String = args.remove(0);
    match command.as_str() {
        "blur" => {
            let next_arg: String = args.remove(0);
            let ammount: f32 = next_arg.parse().expect("Failed to parse blur ammount.");
            commands.blur_amount = ammount;
            commands.blur = true;
        }
        "brighten" => {
            let next_arg: String = args.remove(0);
            let ammount: i32 = next_arg.parse().expect("Failed to parse brighten ammount.");
            commands.brighten_amount = ammount;
            commands.brighten = true;
        }
        "rotate" => {
            let next_arg: String = args.remove(0);
            let ammount: u32 = next_arg.parse().expect("Failed to parse rotate angle.");
            commands.angle = ammount.to_string();
            commands.rotate = true;
        }
        "crop" => {
            let x_string: String = args.remove(0);
            let x: u32 = x_string.parse().expect("Failed to parse x value");
            let y_string: String = args.remove(0);
            let y: u32 = y_string.parse().expect("Failed to parse y value");
            let width_string: String = args.remove(0);
            let width: u32 = width_string.parse().expect("Failed to parse width value");
            let height_string: String = args.remove(0);
            let height: u32 = height_string.parse().expect("Failed to parse height value");
            commands.crop_x = x;
            commands.crop_y = y;
            commands.crop_width = width;
            commands.crop_height = height;
            commands.crop = true;
        }
        "invert" => {
            commands.invert = true;
        }
        "grayscale" => {
            commands.grayscale = true;
        }
        _ => {}
    }
}

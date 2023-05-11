pub mod apply_commands;
pub mod commands;
pub mod match_command;
pub mod match_one_command;

pub fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE AMOUNT");
    println!("brighten INFILE OUTFILE AMOUNT");
    println!("fractal OUTFILE");
    std::process::exit(-1);
}

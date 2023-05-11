use mirage::{
    apply_commands::apply_commands, commands::Commands, match_command::match_command,
    match_one_command::match_one_command, print_usage_and_exit,
};

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }

    let first_arg: String = args.remove(0);

    if !first_arg.ends_with(".png") {
        match_one_command(first_arg, &mut args);
    } else {
        let infile: String = first_arg;
        let outfile: String = args.remove(0);

        let mut commands: Commands = Commands::default();

        loop {
            match_command(&mut args, &mut commands);
            if args.is_empty() {
                break;
            };
        }

        apply_commands(commands, infile, outfile);
    }
}

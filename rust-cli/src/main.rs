use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Burger Builder")
        .author("againxx")
        .version("1.0")
        .about("Helps you build a burger correctly.")
        .arg(Arg::new("style")
            .value_name("BURGER_STYLE")
            .help("What type of burger do you want?")
            .takes_value(true))
        .get_matches();
    println!("{:?}", matches);
}

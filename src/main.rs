use clap::{Arg, Command};
mod configurator;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Command::new("RusticBildge")
        .author("Paprika")
        .about("A lightweight one solution for uploading file with HTTP")
        .arg(
            Arg::new("cnf")
                .long("config")
                .short('c')
                .value_name("FILE")
                .visible_aliases(["conf, configuration"])
                .help("Use the supplied configuration file"),
        )
        .subcommand(
            Command::new("config-generate")
                .about("Generate boilerplate configuration")
                .arg(
                    Arg::new("out")
                        .required(true)
                        .value_name("OUTPUT")
                        .help("The filename to place the boilerplate configuration"),
                ),
        )
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches_from(wild::args());
    let config = configurator::load_config(None);
    println!("{:?}", config);
    server::run_server(config)
}

mod configurator;
mod server;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = configurator::load_config(None);
    println!("{:?}", config);
    server::run_server(config)
}

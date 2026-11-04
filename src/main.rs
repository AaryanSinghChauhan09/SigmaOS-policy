use sigmaos_policy::{Config, PolicyComponent, Component};

fn main() {
    env_logger::init();
    let mut component = PolicyComponent::new(Config::default());
    if let Err(e) = component.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS policy running");
    let _ = component.shutdown();
}

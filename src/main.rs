use clap::Parser;
use x2y::app::Args;
use x2y::error::X2YError;

fn main() -> Result<(), X2YError> {
    env_logger::init();
    let args = Args::parse();
    args.run()?;
    Ok(())
}

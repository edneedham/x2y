use x2y::app::App;
use x2y::error::X2YError;

fn main() -> Result<(), X2YError> {
    env_logger::init();
    let app = App::matches();
    app.run()?;
    Ok(())
}

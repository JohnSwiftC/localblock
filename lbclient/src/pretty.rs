use std::error::Error;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
// Make sure we fix this bad error handling in the future
pub fn show_wallet_names(names: &[String]) -> Result<(), Box<dyn Error + 'static>> {
    let mut stream = StandardStream::stdout(ColorChoice::Auto);

    let mut spec = ColorSpec::new();
    spec.set_fg(Some(Color::Green));
    spec.set_intense(true);
    stream.set_color(&spec)?;

    writeln!(stream, "WALLETS:")?;

    stream.reset()?;

    let mut i = 0;
    while i < names.len() {
        println!("- {}", names[i]);
        i += 1;
    }

    Ok(())
}

pub fn wallet_created_message(name: &str) -> Result<(), Box<dyn Error + 'static>> {
    let mut stream = StandardStream::stdout(ColorChoice::Auto);
    let mut spec = ColorSpec::new();
    spec.set_intense(true);
    let _ = stream.set_color(&spec); // ignore potential error
    write!(stream, "New wallet ");
    spec.set_fg(Some(Color::Green));
    stream.set_color(&spec);
    write!(stream, "{}", name);
    spec.set_fg(None);
    stream.set_color(&spec);
    write!(stream, " created!");

    Ok(())
}
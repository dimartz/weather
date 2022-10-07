pub use api_cmd::*;
use weather::*;

fn main() {
    let args = return_args();
    if let Some(arg) = args {
        if arg.iter().any(|a| a == "-H" || a == "--help") {
            print_help();
            std::process::exit(0);
        } else if arg.iter().any(|a| a == "-V" || a == "--version") {
            println!(
                "\nweather 0.1.0 (2021)
                \nwritten by dimartz"
            );
            std::process::exit(0);
        }
        if arg.iter().any(|a| a == "-F") {
            forecast_daily();
            std::process::exit(0);
        }
    }

    weather();
    std::process::exit(0);
}

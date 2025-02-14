use curl::easy::Easy;
use serde_json::Value;
use std::env;
use std::process::Command;

pub mod api_cmd;

pub struct Config<'c> {
    pub loc: String,
    pub unit: String,
    key: &'c str,
    url: &'c str,
    icon: Symbols<'c>,
}

pub struct Symbols<'c> {
    unit: String,
    hum: &'c str,
    wind: &'c str,
    sunrise: &'c str,
    sunset: &'c str,
    sun: &'c str,
    moon: &'c str,
    clouds: &'c str,
    rain: &'c str,
    drizzle: &'c str,
    fog: &'c str,
    mist: &'c str,
    haze: &'c str,
    snow: &'c str,
    thunderstorm: &'c str,
    dust: &'c str,
    smoke: &'c str,
    u_arrow: &'c str,
    d_arrow: &'c str,
    hot: &'c str,
    cold: &'c str,
    warning: &'c str,
}

impl Default for Config<'_> {
    fn default() -> Self {
        Config {
            loc: String::new(),
            unit: String::from("metric"),
            key: "85a4e3c55b73909f42c6a23ec35b7147",
            url: "https://api.openweathermap.org/data/2.5/",
            icon: Symbols {
                unit: String::from("\u{fa03}"),
                hum: "\u{e373}",
                wind: "\u{1F32C}",
                sunrise: "\u{1F305}",
                sunset: "\u{1F306}",
                sun: "\u{2600}",
                moon: "\u{1F319}",
                clouds: "\u{2601}",
                rain: "\u{1F327}",
                drizzle: "\u{1F328}",
                fog: "\u{1F32B}",
                mist: "\u{1F301}",
                haze: "\u{1F301}",
                snow: "\u{2744}",
                thunderstorm: "\u{26C8}",
                dust: "\u{1F4A8}",
                smoke: "\u{1F525}\u{1F4A8}",
                u_arrow: "\u{2191}",
                d_arrow: "\u{2193}",
                hot: "\u{1F975}",
                cold: "\u{1F976}",
                warning: "\u{26A0}",
            },
        }
    }
}

impl Config<'_> {
    fn set_icon<'c>(&'c self, data: String, period: &str) -> &'c str {
        match data.trim_matches('"') {
            "Clear" => match period {
                "day" => self.icon.sun,
                "night" => self.icon.moon,
                _ => self.icon.warning,
            },
            "Clouds" => self.icon.clouds,
            "Rain" => self.icon.rain,
            "Drizzle" => self.icon.drizzle,
            "Fog" => self.icon.fog,
            "Mist" => self.icon.mist,
            "Haze" => self.icon.haze,
            "Snow" => self.icon.snow,
            "Thunderstorm" => self.icon.thunderstorm,
            "Dust" => self.icon.dust,
            "Smoke" => self.icon.smoke,
            _ => self.icon.warning,
        }
    }
}

fn init<'c>() -> (Config<'c>, Option<Vec<String>>) {
    let mut cfg = Config::default();
    let mut args = return_args();

    if let Some(arg) = &mut args {
        if arg.iter().any(|a| a.contains("-l=")) && arg.iter().any(|a| a == "-I") {
            if let Some(i) = arg.iter().position(|a| a.contains("-l=")) {
                cfg.loc = arg[i]
                    .replace(' ', "+")
                    .trim_start_matches("-l=")
                    .to_string();
                arg.remove(i);
            }
            if let Some(i) = arg.iter().position(|a| a == ("-I")) {
                cfg.unit = "imperial".to_string();
                cfg.icon.unit = "\u{fa04}".to_string();
                arg.remove(i);
            }
            if arg.is_empty() {
                (cfg, None)
            } else {
                (cfg, args)
            }
        } else if arg.iter().any(|a| a.contains("-l=")) || arg.iter().any(|a| a == "-I") {
            if let Some(i) = arg.iter().position(|a| a.contains("-l=")) {
                cfg.loc = arg[i]
                    .replace(' ', "+")
                    .trim_start_matches("-l=")
                    .to_string();
                arg.remove(i);
            } else {
                cfg.loc = auto_location();
                if let Some(i) = arg.iter().position(|a| a == "-I") {
                    cfg.unit = "imperial".to_string();
                    cfg.icon.unit = "\u{fa04}".to_string();
                    arg.remove(i);
                }
            }
            if arg.is_empty() {
                (cfg, None)
            } else {
                (cfg, args)
            }
        } else {
            cfg.loc = auto_location();
            (cfg, args)
        }
    } else {
        cfg.loc = auto_location();
        (cfg, None)
    }
}

pub fn return_args() -> Option<Vec<String>> {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        None
    } else {
        Some(args)
    }
}

pub fn set_the_period(data: &Value) -> &'static str {
    let timezone = data["timezone"].as_i64().unwrap();
    let now = date_command(vec!["+%s"]).trim().parse::<i64>().unwrap() + timezone;
    let sunrise = data["sys"]["sunrise"].as_i64().unwrap() + timezone;
    let sunset = data["sys"]["sunset"].as_i64().unwrap() + timezone;

    if now >= sunrise && now <= sunset {
        "day"
    } else {
        "night"
    }
}

pub fn date_command(args: Vec<&str>) -> String {
    let time = Command::new("date")
        .args(args)
        .output()
        .expect("Failed to execute `/bin/date`!");

    String::from_utf8(time.stdout).unwrap()
}

pub fn openweathermap(api_cmd: &str, cfg: &Config) -> Value {
    let url = format!(
        "{}{}?q={}&appid={}&units={}",
        cfg.url, api_cmd, cfg.loc, cfg.key, cfg.unit
    );
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle.url(&url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    let data = String::from_utf8(data).unwrap();
    let data: Value = serde_json::from_str(&data).unwrap();

    if data["cod"] == 200 || data["cod"] == "200" {
        data
    } else {
        eprintln!("\nError: {}", data["message"]);
        std::process::exit(1);
    }
}

pub fn auto_location() -> String {
    let mut data = Vec::new();
    let mut handle = Easy::new();
    handle
        .url("http://ip-api.com/json/?fields=status,message,countryCode,city")
        .unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    let data = String::from_utf8(data).unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    match &v["status"] {
        Value::String(s) => {
            if s == "success" {
                format!(
                    "{},{}",
                    v["city"].to_string().trim_matches('"'),
                    v["countryCode"].to_string().trim_matches('"')
                )
            } else {
                eprintln!("\nError: {}", v["message"]);
                std::process::exit(1);
            }
        }
        _ => panic!("JSON data returned unreadable!"),
    }
}

pub fn print_help() {
    println!(
        "
    Usage: weather [OPTIONS]
    
    OPTIONS:

    -l=\"[LOCATION]\"         Specify location:
    (use quotes)                             -l=\"[city name]\"
                                             -l=\"[city name],[country code]\"
                            Examples:
                                             -l=\"Córdoba\"
                                             -l=\"Córdoba,AR\"
                            Default: auto localization.

    -I                      Toggle unit system to Imperial.
                            Default: Metric.

    -F                      Toggle extended forecast mode.
   
    -h                      Show humidity data display.

    -w                      Show wind data display.
    
    -d                      Show daylight data display.

    -                       Show only weather display.
    (dash symbol)

    -H, --help              Display usage information.
   
    -V, --version           Display version.
    "
    );
}

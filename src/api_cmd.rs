use crate::*;
use colored::*;
use std::io::{self, Write};

fn init_config<'c>() -> (Config<'c>, Option<Vec<String>>) {
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

pub fn weather() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let (cfg, args) = init_config();
    let data = openweathermap("weather", &cfg);
    
    let icon_w = set_weather_icon(
        data["weather"][0]["main"].to_string(),
        set_the_period(&data),
        &cfg,
    );
    let temp = || {
        format!(
            "{:.1}{}",
            data["main"]["temp"].to_string().parse::<f32>().unwrap(),
            cfg.icon.unit
        )
    };
    let hum = || &data["main"]["humidity"];
    let wind = || match cfg.unit.as_str() {
        "imperial" => {
            format!(
                "{:.0} mph",
                data["wind"]["speed"].to_string().parse::<f32>().unwrap()
            )
        }
        _ => {
            format!(
                "{:.0} km/h",
                data["wind"]["speed"].to_string().parse::<f32>().unwrap() * 3.6
            )
        }
    };
    let sunrise = || {
        date_command(vec![
            format!("--date=@{}", data["sys"]["sunrise"]).as_str(),
            "+%c",
        ])
        .replace(
            &format!(
                " {}",
                date_command(vec![
                    format!("--date=@{}", data["sys"]["sunrise"]).as_str(),
                    "+%Y"
                ])
                .trim()
            ),
            "",
        )
        .trim()
        .to_string()
    };
    let sunset = || {
        date_command(vec![
            format!("--date=@{}", data["sys"]["sunset"]).as_str(),
            "+%c",
        ])
        .replace(
            &format!(
                " {}",
                date_command(vec![
                    format!("--date=@{}", data["sys"]["sunset"]).as_str(),
                    "+%Y"
                ])
                .trim()
            ),
            "",
        )
        .trim()
        .to_string()
    };

    match args {
        Some(args) => {
            write!(handle, "{} {}", icon_w, temp()).unwrap();
            handle.flush().unwrap();
            for arg in args {
                match arg.as_str() {
                    "-h" => {
                        write!(handle, "  {}{}", hum(), cfg.icon.hum).unwrap();
                        handle.flush().unwrap();
                    }
                    "-w" => {
                        write!(handle, "  {} {}", cfg.icon.wind, wind()).unwrap();
                        handle.flush().unwrap();
                    }
                    "-d" => {
                        write!(
                            handle,
                            "  {} {}  {} {}",
                            cfg.icon.sunrise,
                            sunrise(),
                            cfg.icon.sunset,
                            sunset()
                        )
                        .unwrap();
                        handle.flush().unwrap();
                    }
                    "-" => (),
                    _ => eprint!("\t{}: '{}'\t", "Unrecognized option".red(), arg.yellow()),
                }
            }
        }
        None => {
            write!(
                handle,
                "    {} in {}:    {}  {}    {} {}     {}  {}    {} {}  {} {}",
                "Weather".bright_blue().bold(),
                data["name"].to_string().trim_matches('"').green().bold(),
                icon_w,
                temp(),
                hum(),
                cfg.icon.hum,
                cfg.icon.wind,
                wind(),
                cfg.icon.sunrise,
                sunrise(),
                cfg.icon.sunset,
                sunset()
            )
            .unwrap();
            handle.flush().unwrap();
        }
    }
    writeln!(handle).unwrap();
    handle.flush().unwrap();
}

pub fn forecast_daily() {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    let (cfg, mut args) = init_config();
    if let Some(arg) = &mut args {
        if arg.iter().any(|a| a == "-F") {
            if let Some(i) = arg.iter().position(|a| a == "-F") {
                arg.remove(i);
                if arg.is_empty() {
                    args = None;
                }
            }
        }
    }
    let data = openweathermap("forecast/daily", &cfg);

    writeln!(
        handle,
        "\n\n\n\t{} {}:",
        data["city"]["name"]
            .to_string()
            .trim_matches('"')
            .green()
            .bold(),
        "forecast".bold()
    )
    .unwrap();
    handle.flush().unwrap();

    let colorized = |min, max, s: String| match cfg.unit.as_str() {
        "imperial" => {
            if min >= 77.0 {
                format!("{}", s.magenta().bold())
            } else if max <= 59.0 {
                format!("{}", s.cyan().bold())
            } else {
                s
            }
        }
        _ => {
            if min >= 25.0 {
                format!("{}", s.magenta().bold())
            } else if max <= 15.0 {
                format!("{}", s.cyan().bold())
            } else {
                s
            }
        }
    };

    for i in 0..data["list"].as_array().unwrap().len() {
        let min = data["list"][i]["temp"]["min"]
            .to_string()
            .parse::<f32>()
            .unwrap();
        let min_fmt = format!("{:2.0}", min);
        let max = data["list"][i]["temp"]["max"]
            .to_string()
            .parse::<f32>()
            .unwrap();
        let max_fmt = format!("{:2.0}", max);
        let hum = || {
            format!(
                "{:3.0} {}",
                data["list"][i]["humidity"]
                    .to_string()
                    .parse::<f32>()
                    .unwrap(),
                cfg.icon.hum
            )
        };
        let wind = || match cfg.unit.as_str() {
            "imperial" => {
                format!(
                    "{:3.0} mph",
                    data["list"][i]["speed"].to_string().parse::<f32>().unwrap()
                )
            }
            _ => {
                format!(
                    "{:3.0} km/h",
                    data["list"][i]["speed"].to_string().parse::<f32>().unwrap() * 3.6
                )
            }
        };
        let sunrise = || {
            date_command(vec![
                format!("--date=@{}", data["list"][i]["sunrise"]).as_str(),
                "+%T",
            ])
            .trim()
            .to_string()
        };
        let sunset = || {
            date_command(vec![
                format!("--date=@{}", data["list"][i]["sunset"]).as_str(),
                "+%T",
            ])
            .trim()
            .to_string()
        };
        let datetime = date_command(vec![
            format!("--date=@{}", data["list"][i]["dt"]).as_str(),
            "+%c",
        ])
        .replace(
            &format!(
                " {}",
                date_command(vec![
                    format!("--date=@{}", data["list"][i]["dt"]).as_str(),
                    "+%T"
                ])
                .trim()
            ),
            "",
        )
        .replace(
            &format!(
                " {}",
                date_command(vec![
                    format!("--date=@{}", data["list"][i]["dt"]).as_str(),
                    "+%Y"
                ])
                .trim()
            ),
            "",
        )
        .trim()
        .to_string();
        write!(handle, "\n\n\t{}  ", colorized(min, max, datetime)).unwrap();
        handle.flush().unwrap();

        let icon_w = set_weather_icon(
            data["list"][i]["weather"][0]["main"].to_string(),
            "day",
            &cfg,
        );
        write!(handle, "{}   ", icon_w).unwrap();
        handle.flush().unwrap();

        let min_fmt = |min| match cfg.unit.as_str() {
            "imperial" => {
                if min >= 77.0 {
                    format!(
                        "{}{}{}",
                        cfg.icon.d_arrow.red().bold(),
                        min_fmt.red().bold(),
                        cfg.icon.unit.red().bold()
                    )
                } else if min <= 50.0 {
                    format!(
                        "{}{}{}",
                        cfg.icon.d_arrow.blue().bold(),
                        min_fmt.blue().bold(),
                        cfg.icon.unit.blue().bold()
                    )
                } else {
                    format!("{}{}{}", cfg.icon.d_arrow.blue(), min_fmt, cfg.icon.unit,)
                }
            }
            _ => {
                if min >= 25.0 {
                    format!(
                        "{}{}{}",
                        cfg.icon.d_arrow.red().bold(),
                        min_fmt.red().bold(),
                        cfg.icon.unit.red().bold()
                    )
                } else if min <= 10.0 {
                    format!(
                        "{}{}{}",
                        cfg.icon.d_arrow.blue().bold(),
                        min_fmt.blue().bold(),
                        cfg.icon.unit.blue().bold()
                    )
                } else {
                    format!("{}{}{}", cfg.icon.d_arrow.blue(), min_fmt, cfg.icon.unit,)
                }
            }
        };

        let max_fmt = |max| match cfg.unit.as_str() {
            "imperial" => {
                if max >= 86.0 {
                    format!(
                        "{}{}{}",
                        cfg.icon.u_arrow.red().bold(),
                        max_fmt.red().bold(),
                        cfg.icon.unit.red().bold()
                    )
                } else if max <= 59.0 {
                    format!(
                        "{}{}{}",
                        cfg.icon.u_arrow.blue().bold(),
                        max_fmt.blue().bold(),
                        cfg.icon.unit.blue().bold()
                    )
                } else {
                    format!("{}{}{}", cfg.icon.u_arrow.red(), max_fmt, cfg.icon.unit,)
                }
            }
            _ => {
                if max >= 30.0 {
                    format!(
                        "{}{}{}",
                        cfg.icon.u_arrow.red().bold(),
                        max_fmt.red().bold(),
                        cfg.icon.unit.red().bold()
                    )
                } else if max <= 15.0 {
                    format!(
                        "{}{}{}",
                        cfg.icon.u_arrow.blue().bold(),
                        max_fmt.blue().bold(),
                        cfg.icon.unit.blue().bold()
                    )
                } else {
                    format!("{}{}{}", cfg.icon.u_arrow.red(), max_fmt, cfg.icon.unit,)
                }
            }
        };

        let emoji = match cfg.unit.as_str() {
            "imperial" => {
                if min >= 77.0 {
                    cfg.icon.hot
                } else if max <= 59.0 {
                    cfg.icon.cold
                } else {
                    "  "
                }
            }
            _ => {
                if min >= 25.0 {
                    cfg.icon.hot
                } else if max <= 15.0 {
                    cfg.icon.cold
                } else {
                    "  "
                }
            }
        };

        write!(handle, "{}{} {}  ", min_fmt(min), max_fmt(max), emoji).unwrap();
        handle.flush().unwrap();

        match &args {
            Some(args) => {
                for arg in args {
                    match arg.as_str() {
                        "-h" => {
                            write!(handle, "{}      ", colorized(min, max, hum())).unwrap();
                            handle.flush().unwrap();
                        }
                        "-w" => {
                            write!(
                                handle,
                                "{}  {}     ",
                                cfg.icon.wind,
                                colorized(min, max, wind())
                            )
                            .unwrap();
                            handle.flush().unwrap();
                        }
                        "-d" => {
                            write!(
                                handle,
                                "{} {}  {} {}     ",
                                cfg.icon.sunrise,
                                colorized(min, max, sunrise()),
                                cfg.icon.sunset,
                                colorized(min, max, sunset())
                            )
                            .unwrap();
                            handle.flush().unwrap();
                        }
                        "-" => (),
                        _ => eprint!("\t{}: '{}'\t", "Unrecognized option".red(), arg.yellow()),
                    }
                }
            }
            None => {
                write!(
                    handle,
                    "{}      {}  {}     {} {}  {} {}",
                    colorized(min, max, hum()),
                    cfg.icon.wind,
                    colorized(min, max, wind()),
                    cfg.icon.sunrise,
                    colorized(min, max, sunrise()),
                    cfg.icon.sunset,
                    colorized(min, max, sunset())
                )
                .unwrap();
                handle.flush().unwrap();
            }
        }
        writeln!(handle).unwrap();
        handle.flush().unwrap();
    }
    writeln!(handle).unwrap();
    handle.flush().unwrap();
}

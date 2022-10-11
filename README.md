# weather



## Description

Weather is a CLI for displaying the current weather conditions
in your terminal, with support for ANSI colors and Unicode symbols.

![wtr_def_sc](https://user-images.githubusercontent.com/98893034/194855543-759d83ba-6ae9-4c9c-a06b-326079fa9c20.png)

Weather data comes from the `OpenWeatherMap` free weather API.



## Requirements

Weather requires the following dependencies:

- cURL
- Cargo
- Nerd Fonts
- JoyPixels (or any other emoji source)



## Installation

The following is step-by-step instruction.

```
$ git clone https://github.com/dimartz/weather.git
$ cargo install --path weather/
```
Move the binary file `~/.cargo/bin/weather` to the path of your choice.



## Uninstall

If the binary remains on the original path:
```
$ cargo uninstall weather
```



## Usage

### Synopsis

	weather [-l location] [-I imperial system] [-F extended forecast]
	        [-h humidity] [-w wind] [-d daylight] [- only weather]
	        [-H help] [-V version]

### Options

	-l="[LOCATION]"
	(use quotes)		Specify location.
				Default: auto localization.
	
	-I
				Toggle unit system to Imperial.
				Default: Metric.
	
	-F
				Toggle extended forecast mode.
	
	-h
				Show humidity data display.
	
	-w
				Show wind data display.
		
	-d
				Show daylight data display.
		
	-
	(dash symbol)		Show only weather display.
	
	-H, --help
				Display usage information.
		
	-V, --version
				Display version


### Examples

Display weather using imperial units for Gold Coast, Australia:

	weather -l="Gold Coast,AU" -I

![wtr_ex_sc](https://user-images.githubusercontent.com/98893034/194863363-2104ae37-bb65-459b-8ce4-baa351314cd9.png)



### Location

Location format is `city,CC` where `CC` is a two-letter ISO 3166-1 alpha-2
country code. A list of country codes is available [here][1].

Example: `-l="Copenhagen,DK"`

In case no location is specified, weather will auto-detect your location with [ip-api][2].



## License

Weather is released under the BSD 2-Clause license. See `LICENSE` file
for details.



## Resources

GitHub: https://github.com/dimartz/weather
GitHub: https://github.com/fcambus/ansiweather

[1]: https://www.statdns.com/cctlds/
[2]: https://ip-api.com/

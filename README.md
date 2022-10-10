# weather



## Description

Weather is a CLI for displaying the current weather conditions
in your terminal, with support for ANSI colors and Unicode symbols.

![wtr_def_sc](https://user-images.githubusercontent.com/98893034/194855543-759d83ba-6ae9-4c9c-a06b-326079fa9c20.png)

Weather data comes from the `OpenWeatherMap` free weather API.



## Requirements

Weather requires the following dependencies:

- cURL
- Nerd Fonts
- JoyPixels



## Usage

### Synopsis

	weather [-l location] [-I imperial system] [-F extended forecast]
	        [-h humidity] [-w wind] [-d daylight] [- only weather]
	        [-H help] [-V version]

### Options

	-l="[LOCATION]"
	(use quotes)
	        Specify location.
	
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
	(dash symbol)
	        Show only weather display.
	
	-H, --help
		Display usage information.
		
	-V, --version
		Display version.
	        Toggle wind data display ( true or false ).

### Examples

Display extended forecast using metric units for Buenos Aires, Argentina:

	weather -l="Buenos Aires,AR" -F



### OpenWeatherMap API key

Specify an OpenWeatherMap API key. By default AnsiWeather uses its own
key, but users can optionally get their own one by creating a free
[OpenWeatherMap account][12].

	api_key:85a4e3c55b73909f42c6a23ec35b7147



## License

Weather is released under the BSD 2-Clause license. See `LICENSE` file
for details.



## Resources

GitHub: https://github.com/dimartz/weather

# weather



## Description

Weather is a command API for displaying the current weather conditions
in your terminal, with support for ANSI colors and Unicode symbols.

Weather data comes from the `OpenWeatherMap` free weather API.



## Requirements

Weather requires the following dependencies:

- cURL



## Installation

After cloning the repository, simply invoke the script by typing:

	./ansiweather




## Usage

### Synopsis

	ansiweather [-l location] [-u system] [-f days] [-F] [-a value]
	            [-s value] [-k key] [-i value] [-w value] [-h value]
	            [-H value] [-p value] [-d value] [-v]

### Options

	-l location
	        Specify location.
	
	-u system
	        Specify unit system to use ( metric or imperial ).
	
	-f days
	        Toggle forecast mode for the specified number of upcoming days.
	
	-F      Toggle forecast mode for the next five days.
	
	-a value
	        Toggle ANSI colors display ( true or false ).
	
	-s value
	        Toggle symbols display ( true or false ).
	
	-k key  Specify OpenWeatherMap API key.
	
	-i value
	        Toggle UV Index display ( true or false ).
	
	-w value
	        Toggle wind data display ( true or false ).
	
	-h value
	        Toggle humidity data display ( true or false ).
	
	-H value
	        Toggle Feels like display ( true or false ).
	
	-p value
	        Toggle pressure data display ( true or false ).
	
	-d value
	        Toggle daylight data display ( true or false ).
	
	-v      Display version.

### Examples

Display forecast using metric units for the next five days (showing symbols
and daylight data) for Rzeszow, Poland:

	ansiweather -l "Rzeszow,PL" -u metric -s true -f 5 -d true



## Configuration

The default config file is ~/.weatherrc. The environment variable
ANSIWEATHERRC can be set to override this. The following configuration
options (detailed below) are available and should be set according to
your location and preferences.

Example: `~/.ansiweatherrc`

	location:Rzeszow,PL
	units:metric
	show_daylight:true



### Location

Location format is `city,CC` where `CC` is a two-letter ISO 3166-1 alpha-2
country code. A list of country codes is available [here][10].
Alternatively, it's also possible to specify locations by their ID, a city
list is available [here][11].

In case no location is specified, AnsiWeather will fallback to the default
location.

Example: `Rzeszow,PL`

	location:Rzeszow,PL



### System of Units

Both `metric` and `imperial` systems are supported.

	units:metric

Default: `metric`



### Display wind / humidity / pressure

Toggle UV Index, wind, humidity, and/or pressure display. Values can be either
`true` or `false`.

	show_uvi:true
	show_wind:true
	show_humidity:true
	show_pressure:true

Default: `true`

### Display sunrise / sunset

Toggle daylight display. Value can be either `true` or `false`.

	show_daylight:false

Default: `false`



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

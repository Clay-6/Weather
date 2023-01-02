# Weather-Rs

Simple command line application to fetch weather data from [`openweathermap.org`](https://openweathermap.org)'s API

## Usage

1. [Install](#installation) the app with whichever method you prefer.
2. Set an environment variable for your API key. By default it will search for the name `WEATHER_API_KEY`.
   Alternatively, you can use a specific API key by using `config --api-key` to store a specific key or `--use-key`
   to use a key for one session
3. Run the program & enter the name of the location you want to get the info for.

## Installation

### Through Cargo

Make sure you have [rustup](https://rustup.rs) installed & the latest stable Rust version. Then,
run the command

```shell
cargo install weather-rs
```

to install.

### Windows Installer

Download the installer from the [latest GitHub release](https://github.com/Clay-6/Weather-Rs/releases/latest)
and run it. All necessary changes to your `PATH` will be made by the installer.

### Manual

Download the executable from the [latest GitHub release](https://github.com/Clay-6/Weather-Rs/releases/latest)
and place it somewhere on your `PATH`

## Building from Source

Ensure you have the latest stable Rust version installed through [rustup](https://rustup.rs) then `git clone`
the repo & `cd` into it. Then, run `cargo build` for a debug build, or `cargo build --release` for a release
build.

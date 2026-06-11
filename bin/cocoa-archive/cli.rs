use std::env;

pub enum Command {
    Font {
        name: String,
        size: f32,
    },
    Color {
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    },
    Help,
    Version,
}

pub enum GenericArgument {
    Format(Format),
}

pub enum Format {
    Binary,
    Base64,
}

pub fn parse_command() -> Result<(Command, Vec<GenericArgument>), String> {
    let mut args = env::args().skip(1);

    let command = args.next().ok_or("missing command")?;
    let mut generic_arguments = Vec::new();

    match command.as_str() {
        "color" => parse_color(args, &mut generic_arguments).map(|command| (command, generic_arguments)),
        "font" => parse_font(args, &mut generic_arguments).map(|command| (command, generic_arguments)),
        "-h" | "--help" => Ok((Command::Help, generic_arguments)),
        "-V" | "--version" => Ok((Command::Version, generic_arguments)),
        cmd => Err(format!("unknown command: {}", cmd)),
    }
}

fn parse_color(mut args: impl Iterator<Item = String>, generic_arguments: &mut Vec<GenericArgument>) -> Result<Command, String> {
    let mut red: f32 = 1.0;
    let mut green: f32 = 1.0;
    let mut blue: f32 = 1.0;
    let mut alpha: f32 = 1.0;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--red" => red = parse_color_component("red", args.next())?,
            "--green" => green = parse_color_component("green", args.next())?,
            "--blue" => blue = parse_color_component("blue", args.next())?,
            "--alpha" => alpha = parse_color_component("alpha", args.next())?,
            arg => {
                let generic_argument = parse_generic_argument("color", arg, args.next())?;
                generic_arguments.push(generic_argument);
            },
        }
    }

    Ok(Command::Color {
        red,
        green,
        blue,
        alpha,
    })
}

fn parse_color_component(component: &str, value: Option<String>) -> Result<f32, String> {
    let parsed: f32 = value
        .ok_or(format!("missing {} value", component))?
        .trim()
        .parse()
        .map_err(|_| format!("invalid {} value", component))?;

    if !(0.0..=1.0).contains(&parsed) {
        Err(format!("{} value must be between 0.0 and 1.0", component))
    } else {
        Ok(parsed)
    }
}

fn parse_font(mut args: impl Iterator<Item = String>, generic_arguments: &mut Vec<GenericArgument>) -> Result<Command, String> {
    let mut name: Option<String> = None;
    let mut size: f32 = 10.0;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--name" => {
                name = Some(args.next().ok_or("missing font name")?.trim().to_owned())
            }
            "--size" => {
                size = args.next()
                        .ok_or("missing font size")?
                        .trim()
                        .parse()
                        .map_err(|_| "invalid font size")?
            }
            arg => {
                let generic_argument = parse_generic_argument("font", arg, args.next())?;
                generic_arguments.push(generic_argument);
            },
        }
    }

    Ok(Command::Font {
        name: name.ok_or("missing name value")?,
        size,
    })
}

fn parse_generic_argument(cmd: &str, arg: &str, value: Option<String>) -> Result<GenericArgument, String> {
    let Some(value) = value else {
        return Err(format!("missing value for argument: {arg}"));
    };

    match arg {
        "-f" | "--format" => {
            match value.as_str() {
                "base64" => Ok(GenericArgument::Format(Format::Base64)),
                "binary" => Ok(GenericArgument::Format(Format::Binary)),
                _ => Err(format!("invalid format: {}", value)),
            }
        }
        _ => Err(format!("unknown argument \"{}\" for command \"{}\"", arg, cmd)),
    }
}

pub(crate) const HELP: &str = "\
cocoa-archive

Archive supported macOS Cocoa objects as property list data.

Usage:
    cocoa-archive color [--red <RED>] [--green <GREEN>] [--blue <BLUE>] [--alpha <ALPHA>] [--format <FORMAT>]
    cocoa-archive font --name <NAME> [--size <SIZE>] [--format <FORMAT>]
    cocoa-archive --help
    cocoa-archive --version

Commands:
    color    Archive an NSColor
    font     Archive an NSFont

Generic options:
    -f, --format <FORMAT>    Output format, \"binary\" or \"base64\" [default: \"binary\"]
    -h, --help               Print help
    -V, --version            Print version

Color options:
    --red <RED>          Red component, from 0.0 to 1.0 [default: 1.0]
    --green <GREEN>      Green component, from 0.0 to 1.0 [default: 1.0]
    --blue <BLUE>        Blue component, from 0.0 to 1.0 [default: 1.0]
    --alpha <ALPHA>      Alpha component, from 0.0 to 1.0 [default: 1.0]

Font options:
    --name <NAME>        Font name
    --size <SIZE>        Font size in points [default: 10.0]
";
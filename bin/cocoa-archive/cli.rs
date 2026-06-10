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
}

pub fn parse_command() -> Result<Command, String> {
    let mut args = env::args().skip(1);

    let command = args.next().ok_or("missing command")?;
    match command.as_str() {
        "font" => parse_font(args),
        "color" => parse_color(args),
        cmd => Err(format!("unknown command: {}", cmd)),
    }
}

fn parse_font(mut args: impl Iterator<Item = String>) -> Result<Command, String> {
    let mut name: Option<String> = None;
    let mut size: Option<f32> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-n" | "--name" => {
                name = Some(args.next().ok_or("missing font name")?.trim().to_owned())
            }
            "-s" | "--size" => {
                size = Some(
                    args.next()
                        .ok_or("missing font size")?
                        .trim()
                        .parse()
                        .map_err(|_| "invalid font size")?,
                )
            }
            arg => return Err(format!("unknown argument: {}", arg)),
        }
    }

    Ok(Command::Font {
        name: name.ok_or("missing --name")?,
        size: size.ok_or("missing --size")?,
    })
}

fn parse_color(mut args: impl Iterator<Item = String>) -> Result<Command, String> {
    let mut red: f32 = 1.0;
    let mut green: f32 = 1.0;
    let mut blue: f32 = 1.0;
    let mut alpha: f32 = 1.0;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-r" | "--red" => red = parse_color_component("red", args.next())?,
            "-g" | "--green" => green = parse_color_component("green", args.next())?,
            "-b" | "--blue" => blue = parse_color_component("blue", args.next())?,
            "-a" | "--alpha" => alpha = parse_color_component("alpha", args.next())?,
            arg => return Err(format!("unknown argument: {}", arg)),
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

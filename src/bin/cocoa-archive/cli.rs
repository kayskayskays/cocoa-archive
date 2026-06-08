use std::env;

pub enum Command {
    Font { name: String, size: f32 },
}

pub fn parse_command() -> Result<Command, String> {
    let mut args = env::args().skip(1);
    let command = args.next().ok_or("missing command")?;
    parse_command_0(command, args)
}

fn parse_command_0(cmd: String, args: impl Iterator<Item = String>) -> Result<Command, String> {
    match cmd.as_str() {
        "font" => parse_font(args),
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

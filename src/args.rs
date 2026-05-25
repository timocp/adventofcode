use std::collections::VecDeque;

#[derive(Debug)]
pub struct Args {
    pub visualise: bool,
    pub input: Option<String>,
    pub year: Option<u32>,
    pub day: Option<u32>,
}

pub fn parse() -> Result<Args, String> {
    let mut args = Args {
        visualise: false,
        input: None,
        year: None,
        day: None,
    };

    let mut raw: VecDeque<_> = std::env::args().skip(1).collect();
    let mut positional: Vec<String> = Vec::new();

    while let Some(arg) = raw.pop_front() {
        match arg.as_str() {
            "-v" | "--visualise" => args.visualise = true,
            "-i" | "--input" => {
                if let Some(input) = raw.pop_front() {
                    args.input = Some(input);
                } else {
                    return Err(format!("{} requires a file name", arg));
                }
            }
            "--" => {
                positional.extend(raw.clone());
            }
            flag if flag.starts_with('-') => {
                return Err(format!("Unknown flag: {}", flag));
            }
            other => positional.push(other.to_string()),
        }
    }

    let mut it = positional.into_iter();
    if let Some(year_str) = it.next() {
        args.year = Some(
            year_str
                .parse()
                .map_err(|_| format!("Year must be a number, got: {}", year_str))?,
        );
    }
    if let Some(day_str) = it.next() {
        args.day = Some(
            day_str
                .parse()
                .map_err(|_| format!("Day must be a number, got: {}", day_str))?,
        );
    }
    if let Some(extra) = it.next() {
        return Err(format!("Extra argument: {}", extra));
    }

    Ok(args)
}

pub fn exit_usage() -> ! {
    eprintln!("Usage: cargo run -- [-v|--visualise] [-i|--input <file>] <year> [day]");
    std::process::exit(1);
}

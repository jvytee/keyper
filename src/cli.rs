use anyhow::{Context, Result};
use getopts::Options;

pub struct Params {
    pub help: Option<String>,
    pub port: u16,
}

pub fn parse_args(args: &[String]) -> Result<Params> {
    let opts = create_options();
    let matches = opts
        .parse(&args[1..])
        .context("Could not parse arguments")?;

    let help = if matches.opt_present("h") {
        Some(create_help(&args[0]))
    } else {
        None
    };

    let port_str = matches.opt_str("p").unwrap_or("3000".to_string());
    let port: u16 = port_str
        .parse()
        .with_context(|| format!("Could not parse argument {port_str} as valid port number"))?;

    Ok(Params { help, port })
}

pub fn create_help(program: &str) -> String {
    let opts = create_options();
    let brief = format!("Usage: {program} [options]");
    opts.usage(&brief)
}

fn create_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("h", "help", "Show help & exit");
    opts.optopt("p", "port", "Port to listen on", "PORT");

    opts
}

#[cfg(test)]
mod tests {
    use crate::cli::parse_args;

    #[test]
    fn test_parse_args() {
        let args = vec!["keyper".to_string(), "-p".to_string(), "1337".to_string()];
        let params = parse_args(&args).unwrap();
        assert_eq!(params.port, 1337u16);
    }
}

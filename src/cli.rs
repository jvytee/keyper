use anyhow::{Context, Result};
use getopts::Options;

pub struct Params {
    pub port: u16,
}

pub fn parse_args(args: &[String]) -> Result<Params> {
    let mut opts = Options::new();
    opts.optopt("p", "port", "Port to listen on", "PORT");

    let matches = opts
        .parse(&args[1..])
        .context("Could not parse arguments")?;

    let port_str = matches.opt_str("p").unwrap_or("3000".to_string());
    let port: u16 = port_str
        .parse()
        .with_context(|| format!("Could not parse argument {port_str} as valid port number"))?;

    Ok(Params { port })
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

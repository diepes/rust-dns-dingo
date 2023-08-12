use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    process::exit,
};

use ascii::AsciiString;

use crate::dns_types::RecordType;
use std::str::FromStr;

const HELP: &str = "\
dnstest -- domain information gatherer, obviously
USAGE:
  dnstest [OPTIONS] --record-type TYPE NAME
FLAGS:
  -h, --help                Prints help information
OPTIONS:
  -t, --record-type TYPE    Choose the DNS record type (supports A, CNAME, SOA and AAAA) (default A)
  -r, --resolver IP         Which DNS resolver to query (default is 1.1.1.1:53)
  -i, --interval Seconds    If specified repeats and sleeps interval seconds between dns queries.
ARGS:
  NAME A domain name to look up. Remember, these must be ASCII.(Default google.com)
";

/// Values derived from the CLI arguments.
#[derive(Debug)]
pub struct AppArgs {
    pub record_type: RecordType,
    pub name: String,
    pub resolver: SocketAddr,
    pub interval: u64,
}

impl AppArgs {
    pub fn parse() -> Result<Self, pico_args::Error> {
        let mut pargs = pico_args::Arguments::from_env();

        // Help has a higher priority and should be handled separately.
        if pargs.contains(["-h", "--help"]) {
            print!("{}", HELP);
            std::process::exit(0);
        }

        let record_type = pargs
            .opt_value_from_str("--record-type")?
            .xor(pargs.opt_value_from_str("-t")?)
            .unwrap_or(RecordType::A);

        let interval = pargs
            .opt_value_from_str("--interval")?
            .xor(pargs.opt_value_from_str("-i")?)
            .unwrap_or("0".to_string())
            .parse::<u64>()
            .expect("Interval must be in full seconds e.g. 1");

        // I asked some coworkers and they suggested this DNS resolver
        let default_resolver = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 1, 1, 1), 53));
        let resolver = pargs
            .opt_value_from_str("--resolver")?
            .or(pargs.opt_value_from_str("-r")?)
            .unwrap_or(default_resolver);

        let mut name: String = pargs.free_from_str().unwrap_or("google.com".to_string());
        if AsciiString::from_str(&name).is_err() {
            eprintln!("DNS names must be ASCII, and {name} is not.");
            exit(1);
        }
        if !name.ends_with('.') {
            name.push('.');
        }

        let args = AppArgs {
            record_type,
            name,
            resolver,
            interval,
        };

        let remaining = pargs.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unused arguments left: {:?}.", remaining);
        }

        Ok(args)
    }
}

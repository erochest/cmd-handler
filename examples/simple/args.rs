use clap::{ArgMatches, Arg};
use crate::actions::BarType;
use cmd_handler::{CmdArg, CmdArgOption};

pub struct ArgName;

impl CmdArg for ArgName {
    fn name() -> &'static str {
        "name"
    }

    fn build<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name(ArgName::name())
            .long("name")
            .short("n")
            .help("The name.")
            .takes_value(true)
            .default_value("foo")
    }
}

impl<'a> CmdArgOption<'a> for ArgName {
    type Value = String;

    fn value<'b: 'a>(matches: &'a ArgMatches<'b>) -> Self::Value {
        Self::value_raw(matches).map(String::from).unwrap_or_default()
    }
}

pub struct ArgType;

impl CmdArg for ArgType {
    fn name() -> &'static str {
        "type"
    }

    fn build<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name(ArgType::name())
            .long("type")
            .short("t")
            .help("The type of bar.")
            .possible_values(&["sand", "open", "raw", "salad", "iron"])
            .default_value("iron")
    }
}

impl<'a> CmdArgOption<'a> for ArgType {
    type Value = BarType;

    fn value<'b: 'a>(matches: &'a ArgMatches<'b>) -> Self::Value {
        Self::value_raw(matches)
            .map(|value| match value {
                "sand" => BarType::Sand,
                "open" => BarType::Open,
                "raw" => BarType::Raw,
                "salad" => BarType::Salad,
                "iron" => BarType::Iron,
                _ => BarType::Iron,
            })
            .unwrap_or_default()
    }
}

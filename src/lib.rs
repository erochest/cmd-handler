use clap::{ArgMatches, Arg};

pub trait Matcher<'a>: Sized {
    fn with(matches: &'a ArgMatches) -> Option<Self>;
}

pub trait CmdArg {
    fn name() -> &'static str;
    fn build<'a, 'b>() -> Arg<'a, 'b>;
}

pub trait CmdArgFlag: CmdArg {
    fn is_present(matches: &ArgMatches) -> bool {
        matches.is_present(Self::name())
    }
}

pub trait CmdArgOption<'a>: CmdArg {
    type Value;

    fn value<'b: 'a>(matches: &'a ArgMatches<'b>) -> Self::Value;

    fn value_raw<'b: 'a>(matches: &'a ArgMatches<'b>) -> Option<&'a str> {
        matches.value_of(Self::name())
    }
}

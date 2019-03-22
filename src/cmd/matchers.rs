use clap::ArgMatches;
use crate::cmd::args::{ArgName, ArgType, CmdArgOption};
use cmd_handler::BarType;
use cmd_handler::actions::{Foo, Bar};

pub trait Matcher<'a>: Sized {
    fn with(matches: &'a ArgMatches) -> Option<Self>;
}

pub struct FooMatchers<'a> {
    matches: &'a ArgMatches<'a>,
}

impl<'a: 'b, 'b> FooMatchers<'a> {
    pub fn name(&'a self) -> String {
        ArgName::value(self.matches)
    }
}

impl<'a> Matcher<'a> for FooMatchers<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("foo")
            .map(|matches| FooMatchers { matches })
    }
}

impl<'a> From<FooMatchers<'a>> for Foo {
    fn from(matcher: FooMatchers<'a>) -> Self {
        Foo::new(matcher.name())
    }
}

pub struct BarMatchers<'a> {
    matches: &'a ArgMatches<'a>,
}

impl<'a: 'b, 'b> BarMatchers<'a> {
    pub fn name(&'a self) -> String {
        ArgName::value(self.matches)
    }

    pub fn bar_type(&'a self) -> BarType {
        ArgType::value(self.matches)
    }
}

impl<'a> Matcher<'a> for BarMatchers<'a> {
    fn with(matches: &'a ArgMatches) -> Option<Self> {
        matches
            .subcommand_matches("bar")
            .map(|matches| BarMatchers { matches })
    }
}

impl<'a> From<BarMatchers<'a>> for Bar {
    fn from(matchers: BarMatchers<'a>) -> Self {
        Bar::new(matchers.name(), matchers.bar_type())
    }
}
use clap::{app_from_crate, crate_name, crate_version, crate_authors, crate_description, App, Arg, ArgMatches};
use crate::cmd::subcmd::{CmdFoo, CmdBar};
use crate::cmd::matchers::{FooMatchers, BarMatchers, Matcher};

pub struct Handler<'a> {
    matches: ArgMatches<'a>,
}

impl<'a: 'b, 'b> Handler<'a> {
    pub fn build() -> App<'a, 'b> {
        app_from_crate!()
            .arg(Arg::with_name("verbose").takes_value(false).help("Output verbose messages."))
            .subcommand(CmdFoo::build())
            .subcommand(CmdBar::build())
    }

    pub fn parse() -> Handler<'a> {
        Handler {
            matches: Handler::build().get_matches(),
        }
    }

    pub fn foo(&'a self) -> Option<FooMatchers> {
        FooMatchers::with(&self.matches)
    }

    pub fn bar(&'a self) -> Option<BarMatchers> {
        BarMatchers::with(&self.matches)
    }
}
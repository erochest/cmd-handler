use clap::{app_from_crate, crate_name, crate_version, crate_authors, crate_description, ArgMatches, App, Arg};
use crate::command::{CmdFoo, CmdBar};
use crate::matchers::{FooMatchers, BarMatchers};
use cmd_handler::Matcher;

pub struct Handler<'a> {
    matches: ArgMatches<'a>,
}

impl<'a: 'b, 'b> Handler<'a> {
    pub fn build() -> App<'a, 'b> {
        app_from_crate!()
            .arg(Arg::with_name("verbose")
                .takes_value(false)
                .help("Output verbose messages."))
            .subcommand(CmdFoo::build())
            .subcommand(CmdBar::build())
    }

    pub fn parse() -> Handler<'a> {
        Handler {
            matches: Handler::build().get_matches(),
        }
    }

    pub fn foo(&self) -> Option<FooMatchers> {
        FooMatchers::with(&self.matches)
    }

    pub fn bar(&self) -> Option<BarMatchers> {
        BarMatchers::with(&self.matches)
    }
}

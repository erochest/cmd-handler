use clap::{App, SubCommand};
use crate::cmd::args::{ArgName, ArgType, CmdArg};

pub struct CmdFoo;

impl CmdFoo {
    pub fn build<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("foo")
            .about("Foo")
            .visible_alias("f")
            .arg(ArgName::build())
    }
}

pub struct CmdBar;

impl CmdBar {
    pub fn build<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name("bar")
            .about("Bar")
            .visible_alias("b")
            .arg(ArgName::build())
            .arg(ArgType::build())
    }
}
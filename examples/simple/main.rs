use crate::actions::{ActionError, Foo, Bar};
use crate::handler::Handler;

fn main() -> Result<(), ActionError> {
    let handler = Handler::parse();
    invoke_action(&handler)
}

fn invoke_action(handler: &Handler) -> Result<(), ActionError> {
    if let Some(foo_matcher) = handler.foo() {
        Foo::from(foo_matcher).invoke()?;
    }
    if let Some(bar_matcher) = handler.bar() {
        Bar::from(bar_matcher).invoke()?;
    }
    Ok(())
}

mod handler;

mod args;

mod matchers;

mod command;

mod actions;
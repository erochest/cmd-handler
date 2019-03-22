
mod cmd;

use cmd::handler::Handler;
use cmd_handler::actions::{ActionError, Foo, Bar};

fn main() -> Result<(), ActionError> {
    let handler = Handler::parse();
    invoke_action(&handler)
}

fn invoke_action(handler: &Handler) -> Result<(), ActionError> {
    if let Some(foo) = handler.foo() {
        Foo::from(foo).invoke()?;
    }
    if let Some(bar) = handler.bar() {
        Bar::from(bar).invoke()?;
    }
    Ok(())
}

/// This could be in a separate library with no dependency on `clap` or `cmd_handler`.
/// This defines an application error `ActionError` and a couple of action cammands: `Foo` and
/// `Bar`.

use std::{fmt, error};

#[derive(Debug)]
pub enum ActionError {
    FooError,
    BarError,
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ActionError::FooError => write!(f, "action error: foo"),
            ActionError::BarError => write!(f, "action error: bar"),
        }
    }
}

impl error::Error for ActionError {}

pub struct Foo {
    name: String,
}

impl Foo {
    pub fn new(name: String) -> Foo {
        Foo { name }
    }

    pub fn invoke(&self) -> Result<(), ActionError> {
        println!("Foo::invoke '{}'", self.name);
        Ok(())
    }
}

#[derive(Debug)]
pub enum BarType {
    Sand,
    Open,
    Raw,
    Salad,
    Iron,
}

impl Default for BarType {
    fn default() -> Self {
        BarType::Iron
    }
}

pub struct Bar {
    name: String,
    bar_type: BarType,
}

impl Bar {
    pub fn new(name: String, bar_type: BarType) -> Bar {
        Bar { name, bar_type }
    }

    pub fn invoke(&self) -> Result<(), ActionError> {
        println!("Bar::invoke '{}' {:?}", self.name, self.bar_type);
        Ok(())
    }
}

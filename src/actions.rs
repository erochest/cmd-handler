use std::{fmt, error};
use crate::BarType;

#[derive(Debug)]
pub enum ActionError {
    FooError
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            ActionError::FooError => write!(f, "action error: foo"),
        }
    }
}

impl error::Error for ActionError {}

pub struct Foo {
    name: String,
}

impl<'a> Foo {
    pub fn new(name: String) -> Foo {
        Foo { name }
    }

    pub fn invoke(&self) -> Result<(), ActionError> {
        println!("Foo::invoke '{}'", &self.name);
        Ok(())
    }
}

pub struct Bar {
    name: String,
    bar_type: BarType,
}

impl<'a> Bar {
    pub fn new(name: String, bar_type: BarType) -> Bar {
        Bar { name, bar_type }
    }

    pub fn invoke(&self) -> Result<(), ActionError> {
        println!("Bar::invoke '{}' {:?}", self.name, self.bar_type);
        Ok(())
    }
}
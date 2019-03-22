pub mod actions;

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
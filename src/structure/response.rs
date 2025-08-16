#[derive(Debug, PartialEq)]
pub struct response {
    numeric: f64,
    boolean: bool
}

impl response {

    pub fn new() -> Self{
        Self{
            numeric: 0.0,
            boolean: false
        }
    }

    pub fn set_numeric(mut self, value : f64) -> Self{
        self.numeric = value;
        self
    }

    pub fn get_numeric(self) -> f64{
        self.numeric
    }
}
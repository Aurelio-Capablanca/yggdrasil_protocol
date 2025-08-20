#[derive(Debug, PartialEq)]
pub struct Response {
    numeric: f64,
    boolean: bool,
}

impl Response {
    pub fn new() -> Self {
        Self {
            numeric: 0.0,
            boolean: false,
        }
    }

    pub fn define_numeric(mut self, value: f64) -> Self {
        self.numeric = value;
        self
    }

    pub fn define_boolean(mut self, value: bool) -> Self {
        self.boolean = value;
        self
    }

    pub fn get_numeric(self) -> f64 {
        self.numeric
    }

    pub fn get_boolean(self) -> bool {
        self.boolean
    }
}

pub struct Domain {
    status: bool,
    command: String,
    mode: i128,
    precision: i32,
    name: String,
    sub_mode: Vec<Domain>,
}

impl Domain {
    pub fn new() -> Self {
        Self {
            status: true,
            command: "".to_string(),
            mode: 0_i128,
            precision: 0_i32,
            name: "arithmetics".to_string(),
            sub_mode: vec![],
        }
    }

    pub fn get_precision(&self) -> &i32 {
        &self.precision
    }

    pub fn get_mode(&self) -> &i128 {
        &self.mode
    }

    //setting
    pub fn status(mut self, status_in: bool) -> Self {
        self.status = status_in;
        self
    }

    pub fn command(mut self, command_in: String) -> Self {
        self.command = command_in;
        self
    }
}

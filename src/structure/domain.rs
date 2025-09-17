pub struct Domain {
    status: bool,
    command: String,
    mode: i128,
    name: String,
    sub_mode: Vec<Domain>,
}

impl Domain {
    pub fn status(mut self, status_in : bool) -> Self {
        self.status = status_in;
        self
    }

    pub fn command(mut self, command_in : String) -> Self {
        self.command = command_in;
        self
    }
}

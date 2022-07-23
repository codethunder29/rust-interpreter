#[derive(Debug)]
pub struct ScannerError {
    pub msg: String,
    pub line: u32,
    pub pos: u32,
    pub source_line: String
}

impl ScannerError {
    pub fn get_message(&self) -> String {
        self.msg.clone()
    }

    pub fn print_msg(&self) {
        let line_num_len = self.line.to_string().len();
        let mut space_padding = String::new();

        // need to add 2 beacuse of ": "
        for _ in 0..self.pos as usize + line_num_len + 3 {
            space_padding += " ";
        }

        println!("\n{}", self.msg);
        println!("{}: {}", self.line, self.source_line);
        println!("{}", space_padding + "^");
    }
}
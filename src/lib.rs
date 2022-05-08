pub mod parser {
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    use std::path::PathBuf;
    use regex::Regex;
    use self::utils::CommandType;

    pub struct Parser {
        pub actual_command_type: CommandType
    }

    impl Parser {
        pub fn handle(mut self, filename : PathBuf) {
            let file_content = self.load_file(filename);

            let mut output = File::create("foo.hack").expect("Error during dist file creation");

            for line in file_content.lines() {
                let line = line.expect("Unable to read line");
                let assembled_line = self.parse(line);

                match assembled_line {
                    Some(assembled_line) => writeln!(output, "{}", assembled_line).expect("Error during writing into dist file"),
                    None => (),
                }
            }
        }

        fn parse(&mut self, line: String) -> Option<String> {
            if line.starts_with("/") || line.starts_with(" ") {
                return None;
            }
            

            self.decode_command_type(&line);

            let assembled_line = match self.actual_command_type {
                CommandType::ACOMMAND => self.handle_a_command(&line),
                CommandType::CCOMMAND => self.handle_c_command(&line),
                CommandType::LCOMMAND => self.handle_l_command(&line),
                CommandType::NOCOMMAND => return None,
            };

            Some(assembled_line)
        }

        fn decode_command_type(&mut self, line: &String) {
            if line.len() == 0 {
                self.actual_command_type = CommandType::NOCOMMAND
            } else if line.starts_with("@") {
                self.actual_command_type = CommandType::ACOMMAND 
            } else if line.starts_with("(") {
                self.actual_command_type = CommandType::LCOMMAND 
            } else {
                self.actual_command_type = CommandType::CCOMMAND 
            }
        }

        fn handle_a_command(&self, line: &String) -> String {
            let symbol = self.symbol(&line);

            let parsed_int = symbol.parse::<i16>().expect("Error during symbol parsing");

            let binary = format!("{:b}", parsed_int);
            let padded_binary = format!("{:0>16}", binary);

            padded_binary.to_string()
        }

        fn handle_l_command(&self, line: &String) -> String {
            let symbol = self.symbol(&line);
            symbol
        }

        fn handle_c_command(&self, line: &String) -> String {
            let (jump, line) = if line.contains(";") {
                let line_parts: Vec<&str> =line.splitn(2, ';').collect();

                (line_parts[1], line_parts[0])
            } else {
                ("", line.as_str())
            };

            let (dest, comp, _line) = if line.contains("=") {
                let line_parts : Vec<&str> = line.splitn(2, '=').collect();

                let dest = line_parts[0]; 
                let comp = line_parts[1]; 


                (dest, comp, line)
            } else {
                let comp = line;

                ("", comp, line)
            };

            let operate_on_a_or_m = if comp.contains("M") {
                "1"
            } else {
                "0"
            };

            let jump = self.resolve_jump(&jump);
            let dest = self.resolve_dest(&dest);
            let comp = self.resolve_comp(&comp);

            format!("111{}{}{}{}", operate_on_a_or_m, comp, dest, jump)
        }

        fn resolve_jump(&self, jump: &str) -> &str {
            match jump {
                "JGT" => "001",
                "JEQ" => "010",
                "JGE" => "011",
                "JLT" => "100",
                "JNE" => "101",
                "JLE" => "110",
                "JMP" => "111",
                _ => "000"
            }
        }

        fn resolve_dest(&self, dest: &str) -> &str {
            match dest {
                "M" => "001",
                "D" => "010",
                "MD" => "011",
                "A" => "100",
                "AM" => "101",
                "AD" => "110",
                "AMD" => "111",
                _ => "000",
            }
        }

        fn resolve_comp(&self, comp: &str) -> &str {
            match  comp {
                "0" => "101010",
                "1" => "111111",
                "-1" => "111010",
                "D" => "001100",
                "A" | "M" => "110000",
                "!D" => "001101",
                "!A" | "!M" => "110001",
                "-D" => "001111",
                "-A" | "-M" => "110011",
                "D+1" => "011111",
                "A+1" | "M+1" => "110111",
                "D-1" => "001110",
                "A-1" | "M-1" => "110010",
                "D+A" | "D+M" => "000010",
                "D-A" | "D-M" => "010011",
                "A-D" | "M-D" => "000111",
                "D&A" | "D&M" => "000000",
                "D|A" | "D|M" => "010101",
                _ => "Error during comp resolution",
            }
        }

        fn symbol(&self, line: &String) -> String {
            let re = Regex::new(r"[@\(\)]").expect("Error during regex construction");
            let result = re.replace_all(line, "");

            String::from(result)
        }

        fn load_file(&self, filepath: PathBuf) -> BufReader<File> {
            let file = File::open(filepath).expect("Unable to open file");
            BufReader::new(file)
        }
    }

    pub mod utils {
        pub enum CommandType {
            NOCOMMAND,
            ACOMMAND,
            CCOMMAND,
            LCOMMAND,
        }
    }
}

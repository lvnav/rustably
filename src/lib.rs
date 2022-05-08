pub mod parser {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{prelude::*, BufReader};
    use std::path::PathBuf;
    use regex::Regex;
    use self::utils::CommandType;

    pub struct Parser;

    impl Parser {
        pub fn handle(self, filename : PathBuf) -> String {
            let file_content = self.load_file(&filename);


            let mut symbol_table = SymbolTable::new();
            let mut i : i32 = 0;
            for line in file_content.lines() {
                let line = line.expect("Unable to read line during symbol pass");

                let command_type = self.decode_command_type(&line);

                match command_type {
                    CommandType::ACOMMAND | CommandType::CCOMMAND => i += 1,
                    CommandType::LCOMMAND => {
                        let re = Regex::new(r"\(|\)").expect("Error during regex construction");
                        let line = re.replace_all(line.as_str(), "").trim().to_string();

                        symbol_table.symbol_table.entry(line).or_insert(i);
                    },
                    _ => ()
                };
            }

            let file_content = self.load_file(&filename);
            let mut assembled_string = String::new();
            for line in file_content.lines() {
                let line = line.expect("Unable to read line during assembling pass");

                let assembled_line = self.parse(line, &mut symbol_table);

                match assembled_line {
                    Some(assembled_line) => {
                        assembled_string.push_str(assembled_line.as_str());
                        assembled_string.push_str("\n");
                    },
                    None => (),
                }
            }

            return assembled_string;
        }

        fn parse(&self, line: String, symbol_table: &mut SymbolTable) -> Option<String> {
            let line = line.trim();
            let re = Regex::new(r"//.+").expect("Error during regex construction");
            let result = re.replace_all(line, "");

            let line = result.trim().to_string();

            let command_type = self.decode_command_type(&line);
            let assembled_line = match command_type {
                CommandType::ACOMMAND => self.handle_a_command(&line, symbol_table),
                CommandType::CCOMMAND => self.handle_c_command(&line),
                CommandType::LCOMMAND => return None,
                CommandType::NOCOMMAND => return None,
            };

            Some(assembled_line)
        }

        fn decode_command_type(&self, line: &String) -> CommandType {
            if line.len() == 0 {
                return CommandType::NOCOMMAND
            } else if line.starts_with("@") {
                return CommandType::ACOMMAND 
            } else if line.starts_with("(") {
                return CommandType::LCOMMAND 
            } else if line.starts_with("/") {
                return CommandType::NOCOMMAND
            } else {
                return CommandType::CCOMMAND 
            }
        }

        fn handle_a_command(&self, line: &String, symbol_table: &mut SymbolTable) -> String {
            let symbol = self.symbol(&line);

            let parsed_int = if symbol.parse::<i32>().is_err() {
                if !symbol_table.symbol_table.contains_key(&symbol) {
                    symbol_table.symbol_table.entry(symbol.to_owned()).or_insert(symbol_table.next_available_ram_slot);
                    symbol_table.next_available_ram_slot += 1;
                } 

                symbol_table.symbol_table.get(&symbol).expect("retrieving symbol error").to_owned()
            } else {
                symbol.parse::<i32>().expect("error during parsing symbol")
            };

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

            let (dest, comp, line) = if line.contains("=") {
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

            result.to_string()
        }

        fn load_file(&self, filepath: &PathBuf) -> BufReader<File> {
            let file = File::open(filepath).expect("Unable to open file");
            BufReader::new(file)
        }

    }

    pub mod utils {
        #[derive(Debug)]
        pub enum CommandType {
            NOCOMMAND,
            ACOMMAND,
            CCOMMAND,
            LCOMMAND,
        }
    }

    struct SymbolTable {
        symbol_table: HashMap<String, i32>,
        next_available_ram_slot: i32
    }

    impl SymbolTable {
        fn new() -> SymbolTable {
            SymbolTable { 
                symbol_table : 
                    HashMap::from([
                                  ("SP".to_string(), 0),
                                  ("LCL".to_string(), 1),
                                  ("ARG".to_string(), 2),
                                  ("THIS".to_string(), 3),
                                  ("THAT".to_string(), 4),
                                  ("SCREEN".to_string(), 16384),
                                  ("KBD".to_string(), 24576),
                                  ("R0".to_string(), 0),
                                  ("R1".to_string(), 1),
                                  ("R2".to_string(), 2),
                                  ("R3".to_string(), 3),
                                  ("R4".to_string(), 4),
                                  ("R5".to_string(), 5),
                                  ("R6".to_string(), 6),
                                  ("R7".to_string(), 7),
                                  ("R8".to_string(), 8),
                                  ("R9".to_string(), 9),
                                  ("R10".to_string(), 10),
                                  ("R11".to_string(), 11),
                                  ("R12".to_string(), 12),
                                  ("R13".to_string(), 13),
                                  ("R14".to_string(), 14),
                                  ("R15".to_string(), 15),
                                  ]),

                                  next_available_ram_slot: 16
            }
        }
    }

}

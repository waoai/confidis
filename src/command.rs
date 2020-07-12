use fasthash::metro;
use std::fmt;

#[derive(Debug)]
pub enum CommandType {
    Invalid,
    Set,
    GetAnswer,
    GetSource,
    Believe,
    Configure,
}

impl Default for CommandType {
    fn default() -> CommandType {
        CommandType::Invalid
    }
}

#[derive(Debug, Default)]
pub struct Command {
    pub cmd: CommandType,
    pub source: Option<String>,
    pub distribution: String,
    pub question: Option<String>,
    pub answer: Option<String>,
    pub config_key: Option<String>,
    pub config_val: Option<String>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cmd {
            CommandType::Set => write!(
                f,
                "SET {} {} FROM {}",
                &self.question.as_ref().unwrap(),
                &self.answer.as_ref().unwrap(),
                &self.source.as_ref().unwrap()
            ),
            CommandType::GetAnswer => {
                write!(f, "GET ANSWER TO {}", &self.question.as_ref().unwrap())
            }
            CommandType::GetSource => write!(f, "GET SOURCE {}", &self.question.as_ref().unwrap()),
            CommandType::Believe => write!(f, "BELIEVE {}", &self.source.as_ref().unwrap()),
            CommandType::Configure => write!(
                f,
                "CONFIGURE {} {}",
                &self.config_key.as_ref().unwrap(),
                &self.config_val.as_ref().unwrap()
            ),
            CommandType::Invalid => write!(f, "INVALID"),
        }
    }
}

impl Command {
    pub fn from(line: &str) -> Command {
        // TODO shouldn't split up quoted strings
        let items: Vec<&str> = line.split_whitespace().collect();
        match items[0] {
            "SET" | "set" => {
                // SET <question> <answer> FROM <source>
                Command {
                    cmd: CommandType::Set,
                    question: Some(String::from(items[1])),
                    distribution: String::from("default"),
                    answer: Some(String::from(items[2])),
                    source: Some(String::from(items[4])),
                    ..Default::default()
                }
            }
            "GET" | "get" => {
                if items[1] == "ANSWER" && items[2] == "TO" {
                    // GET ANSWER TO <question>
                    Command {
                        cmd: CommandType::GetAnswer,
                        question: Some(String::from(items[3])),
                        distribution: String::from("default"),
                        ..Default::default()
                    }
                } else if items[1] == "SOURCE" {
                    // GET SOURCE <source>
                    Command {
                        cmd: CommandType::GetSource,
                        source: Some(String::from(items[2])),
                        distribution: String::from("default"),
                        ..Default::default()
                    }
                } else {
                    panic!("Invalid GET command: \"{}\"", line);
                }
            }
            "BELIEVE" | "believe" => {
                // BELIEVE <source>
                Command {
                    cmd: CommandType::Believe,
                    source: Some(String::from(items[1])),
                    distribution: String::from("default"),
                    ..Default::default()
                }
            }
            "CONFIGURE" | "configure" => {
                // CONFIGURE <key> <value>
                Command {
                    cmd: CommandType::Configure,
                    config_key: Some(String::from(items[1])),
                    config_val: Some(String::from(items[2])),
                    ..Default::default()
                }
            }
            _ => panic!("Invalid command: {}", items[0]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub hash: u64,
    pub content: String,
    pub source: String,
}

impl Answer {
    pub fn new(content: String, source: String) -> Self {
        Answer {
            hash: metro::hash64(content.as_bytes()),
            content: content,
            source: source,
        }
    }
}

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

use std::io;

pub struct Entrada {
    stdin: io::Stdin,
}

impl Entrada {
    pub fn nova(stdin: io::Stdin) -> Entrada {
        Entrada { stdin }
    }

    pub fn ler(&self) -> Option<String> {
        let mut buf = String::new();
        match self.stdin.read_line(&mut buf) {
            Ok(_) => Some(buf),
            Err(_) => None,
        }
    }

    pub fn ler_num(&self) -> Option<i32> {
        match self.ler() {
            Some(sbuf) => match sbuf.trim().parse() {
                Ok(num) => Some(num),
                Err(_) => None,
            },
            None => None,
        }
    }
}

#[derive(Debug)]
pub enum OPCODE {
    CLEAR,
    RET,
    JUMP(usize),
    CALL(usize),

    UNKNOWN,
}

pub fn lex (buffer: Vec<String>) -> Vec<OPCODE> {
    let mut vec: Vec<OPCODE> = vec!();
    for string in buffer {
        vec.push(lex_token(string))
    }
    vec
}

fn lex_token(text: String) -> OPCODE {
    let parts: Vec<&str> = text.split_whitespace().collect();
    println!("{:?}", parts);
    match parts.as_slice() {
        ["CLEAR", ] => {
            return OPCODE::CLEAR;
        }

        ["RET", ] => {
            return OPCODE::RET;
        }

        ["JUMP", addr] => {
            if let Ok(data) = addr.parse::<usize>() {
                return OPCODE::JUMP(data);
                // Ajoutez le code pour traiter l'instruction JUMP ici
            } else {
                panic!("Erreur dans le parse d'un JUMP");
            }
        }

        ["CALL", addr] => {
            if let Ok(data) = addr.parse::<usize>() {
                return OPCODE::CALL(data);
                // Ajoutez le code pour traiter l'instruction JUMP ici
            } else {
                panic!("Erreur dans le parse d'un JUMP");
            }
        }

        _ => {
            return OPCODE::UNKNOWN;
        }
    }
}

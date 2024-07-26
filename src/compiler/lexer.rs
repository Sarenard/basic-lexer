#[derive(Debug, PartialEq, Clone)]
pub enum OPCODE {
    MACRO,
    END,
    JUMPLABEL(String),
    JUMP(String),

    UNKNOWN(String),
}

pub fn lex(buffer: Vec<String>) -> Vec<OPCODE> {
    // we build the Vec<OPCODE>
    let mut vec: Vec<OPCODE> = vec!();
    for string in buffer {
        vec.push(lex_token(string));
    }
    // we parse the macros
    let vec = parse_macros(vec);
    // we do jumplabel shit
    let vec = parse_jumplabel(vec);
    vec
}

pub fn parse_jumplabel(old: Vec<OPCODE>) -> Vec<OPCODE> {
    let mut jumplabels: Vec<String> = vec![];
    for opcode in &old {
        match opcode {
            OPCODE::JUMPLABEL(name) => {
                jumplabels.push(name.clone());
            }
            _ => {

            }
        }
    }
    let mut new_vec: Vec<OPCODE> = vec![];
    for item in old {
        match item {
            OPCODE::UNKNOWN(name) if jumplabels.contains(&name.clone()) => {
                new_vec.push(OPCODE::JUMP(name.clone()));
            }
            other => {
                new_vec.push(other);
            }
        }
    }
    new_vec
}

fn parse_macros(old: Vec<OPCODE>) -> Vec<OPCODE> {
    use std::collections::HashMap;
    let mut macros: HashMap<String, Vec<OPCODE>> = HashMap::new();
    let mut macro_buff: Vec<OPCODE> = vec![];
    let mut inside = false;
    for opcode in old {
        if inside {
            macro_buff.push(opcode.clone())
        } 
        match opcode {
            OPCODE::MACRO => {
                inside = true;
            },
            OPCODE::END => {
                let name = macro_buff[0].clone();
                match name {
                    OPCODE::UNKNOWN(name) => {
                        macro_buff.remove(0);
                        macro_buff.remove(macro_buff.len()-1);
                        macros.insert(name, macro_buff);
                        macro_buff = vec![];
                    }
                    _ => {
                        panic!("Error in the macro")

                    }
                }
                inside = false;
            }
            _ => {

            }
        }
    }
    // we replace the macros
    let mut new_vec: Vec<OPCODE> = macros.get("main").unwrap_or(&vec![]).to_vec();
    while macros.keys().any(|key| new_vec.contains(&OPCODE::UNKNOWN(key.clone()))) {
        for (string, opcodes) in macros.iter() {
            new_vec = replace_occurrences(new_vec, OPCODE::UNKNOWN(string.clone()), opcodes);
        }
    }
    new_vec
}

fn replace_occurrences(original: Vec<OPCODE>, target: OPCODE, replacement: &Vec<OPCODE>) -> Vec<OPCODE> {
    let mut result = Vec::new();

    for value in original {
        if value == target {
            result.extend(replacement.iter().cloned());
        } else {
            result.push(value);
        }
    }

    result
}

fn lex_token(text: String) -> OPCODE {
    match text.as_str() {
        "macro" => {
            return OPCODE::MACRO;
        }

        "end" => {
            return OPCODE::END;
        }

        _ if text.ends_with(':') => {
            return OPCODE::JUMPLABEL(text.trim_end_matches(':').to_string());
        }

        unknown => {
            return OPCODE::UNKNOWN(unknown.to_string());
        }
    }
}

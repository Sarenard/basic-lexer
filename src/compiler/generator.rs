use std::collections::HashMap;

use crate::compiler::lexer::OPCODE;

pub fn generate(opcodes: Vec<OPCODE>) -> Vec<u8> {
    let mut bytearray: Vec<u8> = vec![];

    let mut ids: HashMap<String, usize> = HashMap::new();

    let mut index = 0;
    let mut true_index = 0;

    while true_index < opcodes.len() {
        let opcode = opcodes[true_index].clone();
        match opcode {
            OPCODE::CLEAR | OPCODE::JUMP(_) => {
                index += 1;
                true_index += 1;
            }
            OPCODE::JUMPLABEL(name) => {
                ids.insert(name, index);
                true_index += 1;
            }
            OPCODE::MACRO | OPCODE::UNKNOWN(_) | OPCODE::END => {
                panic!("That shouldnt be here !")
            }
        }
    }

    #[cfg(debug_assertions)]
    println!("{:?}", ids);

    for opcode in opcodes {
        match opcode {
            OPCODE::CLEAR => {
                bytearray.extend_from_slice(&[0x00, 0xe0]);
            }
            
            OPCODE::JUMP(name) => {
                let addr = ids.get(&name).unwrap() + 0x200;
                bytearray.extend_from_slice(&[(16 + addr / 256) as u8, (addr % 256) as u8]);
            }

            OPCODE::JUMPLABEL(_) => {
                // ignore
            }

            other => {
                panic!("Unexpected OPCODE : {:?}", other);
            }
        }
    } 

    bytearray
}
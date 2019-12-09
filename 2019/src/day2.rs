use crate::loader::vec_from;

pub fn part1()  -> Vec<i32> {
    let mut opcodes = vec_from(module_path!());
    let mut current_ptr = 0;

    while opcodes[current_ptr] != 99 {
        let fst_ptr = opcodes[current_ptr + 1] as usize;
        let snd_ptr = opcodes[current_ptr + 2] as usize;
        let target = opcodes[current_ptr + 3] as usize;

        match opcodes[current_ptr]{
            1 => {
                opcodes[target] =  opcodes[fst_ptr] + opcodes[snd_ptr];
                current_ptr += 4;
            },
            2 => {
                opcodes[target] = opcodes[fst_ptr] * opcodes[snd_ptr];
                current_ptr += 4;
            }
            _ => { break; }
        }
    }

    opcodes
}

use crate::loader::vec_from;

pub fn part1() -> i32 {
    let mut opcodes = vec_from(module_path!());
    opcodes[1] = 12;
    opcodes[2] = 2;
    computer(opcodes)
}

fn computer(input: Vec<i32>) -> i32 {
    let mut memory = input;
    let mut current_ptr = 0;

    #[allow(irrefutable_let_patterns)]
    while let command = memory[current_ptr] {
        let fst_ptr = memory[current_ptr + 1] as usize;
        let snd_ptr = memory[current_ptr + 2] as usize;
        let target = memory[current_ptr + 3] as usize;

        if command == 1 {
            memory[target] =  memory[fst_ptr] + memory[snd_ptr];
            current_ptr += 4;
        } else if command == 2 {
            memory[target] = memory[fst_ptr] * memory[snd_ptr];
            current_ptr += 4;
        } else {
            break;
        }
    }

    memory[0]
}

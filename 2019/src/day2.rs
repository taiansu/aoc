use crate::loader::read_priv;

pub fn part1() -> i32 {
    let mut opcodes = input_vec();
    opcodes[1] = 12;
    opcodes[2] = 2;
    compute(opcodes)
}

pub fn part2() -> i32 {
    let ori_opcodes = input_vec();
    find_result(ori_opcodes, 19_690_720).unwrap_or(0)
}

fn find_result(opcodes: Vec<i32>, result: i32) -> Option<i32> {
    for i in 0..99 {
        for j in 0..99 {
            let mut opcodes = opcodes.to_owned();
            opcodes[1] = i;
            opcodes[2] = j;
            if compute(opcodes) == result {
                return Some(i * 100 + j);
            }
        }
    };
    None
}

fn compute(input: Vec<i32>) -> i32 {
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

fn input_vec() -> Vec<i32> {
    let content = read_priv(module_path!());
    content.trim_end().split(",").map(|s| s.parse::<i32>().unwrap()).collect()
}

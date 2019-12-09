use std:: fs;

pub fn lines_from(module_name: &str) -> Vec<String> {
    let content = read_priv(module_name);
    content.trim_end().split("\n").map(|s| String::from(s)).collect()
}

pub fn vec_from(module_name: &str) -> Vec<i32> {
    let content = read_priv(module_name);
    content.trim_end().split(",").map(|s| s.parse::<i32>().unwrap()).collect()
}

fn read_priv(module_name: &str) -> String {
    let option_num: Vec<&str> = module_name.split("::").collect();

    match option_num.get(1) {
        Some(module_num) => {
            fs::read_to_string(format!("priv/2019/{}.in", module_num)).unwrap()
        },
        None => {
            println!("{}", "File Not found");
            String::from("")
        }
    }
}

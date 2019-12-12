use std:: fs;

pub fn read_priv(module_name: &str) -> String {
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

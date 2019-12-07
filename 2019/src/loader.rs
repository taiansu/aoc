use std:: fs;

pub fn from(module_name: &str) -> Vec<String> {
    let option_name: Vec<&str> = module_name.split("::").collect();

    match option_name.get(1) {
        Some(name) => {
            let input = fs::read_to_string(format!("priv/2019/{}.in", name)).unwrap();
            input.split("\n").filter(|&s| s != "").map(|s| String::from(s)).collect()
        },
        None => {
            println!("{}", "File Not found");
            vec![String::from("")]
        }
    }
}

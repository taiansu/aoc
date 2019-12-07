use crate::loader::from;

pub fn exec() {
    let input = from(module_path!());
    let result: i32 = input.iter()
                      .map(|s| s.parse::<f32>().unwrap())
                      .map(|f| calc(f))
                      // .collect();
                      .sum();
    println!("{:?}", result);
}

fn calc(num: f32) -> i32 {
    (num / 3.0).floor() as i32 - 2
}

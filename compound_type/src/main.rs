fn main() {
    // 字符串演示
    str_ts()
}

fn str_ts() {
    let a: String = String::from("ABC");
    let b: &str = "DEF";

    let b: String = b.to_string();

    println!("a is {}", a);
    println!("b is {}", b);
}

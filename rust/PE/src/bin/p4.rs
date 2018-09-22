fn main() {
    let mut vec: Vec<u32> = Vec::new();
    // let mut s: String = "kanishk".to_string().chars().rev().collect();
    for i in (900..=999).rev() {
        for j in (900..=999).rev() {
            let mul = i * j;
            let s: String = mul.to_string()
            .chars()
            .rev()
            .collect();
            let s = s.parse::<u32>()
            .unwrap();
            if s == mul {
                vec.push(mul);
            }
        }
    }
    match vec.iter().max() {
        Some(x) => println!("{}", x),
        _ => println!("Error"),
    }
}

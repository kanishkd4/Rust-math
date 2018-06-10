
fn main() {
    untitled1(1000)
}

pub fn untitled1(upper_bound: u32) {
    let mut s: u32 = 0;
    for n in 1..upper_bound {
        if n % 3 == 0 || n % 5 == 0 {
        s += n;
        }
    }
    println!("{}", s);
}
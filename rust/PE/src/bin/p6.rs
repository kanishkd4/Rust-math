fn main() {
    naive(100);
}

fn naive(number: u32) {
    let mut vec: Vec<u32> = (1..=number).collect();
    let vec_sum: u32 = vec.iter().sum();
    let vec_sum_square = vec_sum.pow(2);
    
    let vec_square_sum: Vec<u32> = vec.iter().map(|x| x.pow(2)).collect();
    let vec_square_sum: u32 = vec_square_sum.iter().sum();
    let answer: u32 = vec_sum_square - vec_square_sum;
    println!("{}", answer);
}
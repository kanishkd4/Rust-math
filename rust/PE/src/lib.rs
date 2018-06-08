
pub fn untitled1(upper_bound: u32) {
    let mut s: u32 = 0;
    for n in 1..upper_bound {
        if n % 3 == 0 || n % 5 == 0 {
        s += n;
        }
    }
    println!("{}", s);
}

pub fn untitled2(upper_bound: usize) {
    let mut v = vec![1, 2];
    for n in 2..upper_bound {
        let new = v[n-1] + v[n-2]; // cannot use this directly inside a mutable borrow as it voids borrowing rules
        &v.push(new);
        if v[n] > 4000000 {
            break
        }
    }
    for n in &mut v {
        if *n % 2 != 0 {
            *n = 0;
        }
    }
    let mut s = 0;
    for n in v {
        s += n;
    }
    println!("{}", s);
}



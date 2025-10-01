fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }
    let mut idx: i32 = 3;
    loop {
        if idx * idx > x {
            return true;
        }
        if x % idx == 0 {
            return false;
        }
        idx += 1;
    }
}
fn p1() {
    for i in 0..101 {
        if is_prime(i) {
            println!("{i}");
        }
    }
}

fn is_coprime(mut a: i32, mut b: i32) -> bool {
    let mut r: i32 = a % b;
    loop {
        if r == 0 {
            break;
        }
        a = b;
        b = r;
        r = a % b;
    }
    return b == 1;
}
fn p2() {
    for i in 1..101 {
        for j in 1..101 {
            println!("{i} si {j} sunt coprime: {}", is_coprime(i, j));
        }
    }
}

fn p3() {
    for i in (2..100).rev() {
        let j: i32 = i - 1;
        println!("{i} bottles of beer on the wall,");
        println!("{i} bottles of beer.");
        println!("Take one down, pass it around,");
        if j > 1 {
            println!("{j} bottles of beer on the wall.");
        } else {
            println!("1 bottle of beer on the wall.");
        }
    }

    println!("1 bottle of beer on the wall,");
    println!("1 bottle of beer.");
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
}
fn main() {
    p1();

    p2();

    p3();
}

fn add_chars_n(mut s : String, c : char, n : i8) -> String
{
    for _i in 0..n
    {
        s.push(c);
    }

    s
}

fn p1()
{
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

fn add_chars_n_reference(s : &mut String, c : char, n : i8)
{
    for _i in 0..n
    {
        s.push(c);
    }
}

fn p2()
{
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n_reference(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

fn add_space(s : &mut String, n : i8)
{
    for _ in 0..n
    {
        s.push(' ');
    }
}

fn add_str(s : &mut String, aux : &str)
{
    s.push_str(aux);
}

fn add_integer(s : &mut String, mut n : i32)
{
    let mut reverse_string : String = String::from("");
    while n != 0
    {
        let digit : u8 = (n % 10) as u8;
        //println!("\nPushed digit {digit} from integer {n}\nDigit as char: {}", digit as char);
        n /= 10;
        reverse_string.push((digit + b'0') as char);
    }
    let mut digit_count : i8 = 0;
    //println!("\nReverse_string for integer is {reverse_string}");
    for c in reverse_string.chars().rev()
    {
        if digit_count >= 3
        {
            digit_count = 0;
            s.push('_');
        }
        s.push(c);
        digit_count += 1;
    }
}

fn add_float(s : &mut String, mut f : f32)
{
    let mut multiply_count : i8 = 0;

    let epsilon : f32 = 0.001;

    while f.fract() > epsilon
    {
        //println!("\nValue of float is {f}");
        multiply_count += 1;
        f *= 10.0;
    }

    let put_dot : bool = multiply_count != 0; // in caz ca nu are nimic fractionar

    let mut float_integer : i32 = f as i32;

    let mut reversed_float_string : String = String::from("");

    while float_integer != 0
    {
        let digit : u8 = (float_integer % 10) as u8;
        reversed_float_string.push((digit + b'0') as char);
        float_integer /= 10;
        multiply_count -= 1;
        if multiply_count == 0 && put_dot
        {
            reversed_float_string.push('.');
        }
    }

    for c in reversed_float_string.chars().rev()
    {
        s.push(c);  
    }
}

fn p3()
{
    let mut res : String = String::from("");
    add_space(&mut res, 40);
    add_str(&mut res, "I 💚\n");
    add_space(&mut res, 40);
    add_str(&mut res, "RUST.\n\n");
    add_space(&mut res, 4);
    add_str(&mut res, "Most");
    add_space(&mut res, 12);
    add_str(&mut res, "crate");
    add_space(&mut res, 6);
    add_integer(&mut res, 306437968);
    add_space(&mut res, 11);
    add_str(&mut res, "and");
    add_space(&mut res, 5);
    add_str(&mut res, "lastest");
    add_space(&mut res, 9);
    add_str(&mut res, "is\n");
    add_space(&mut res, 9);
    add_str(&mut res, "downloaded");
    add_space(&mut res, 8);
    add_str(&mut res, "has");
    add_space(&mut res, 13);
    add_str(&mut res, "downloads");
    add_space(&mut res, 5);
    add_str(&mut res, "the");
    add_space(&mut res, 9);
    add_str(&mut res, "version");
    add_space(&mut res, 4);
    add_float(&mut res, 2.038);
    add_str(&mut res, ".\n");
    add_space(&mut res, 20);
    print!("{res}");
}

fn main()
{
    p1();
    println!();
    p2();
    println!();
    p3();
}
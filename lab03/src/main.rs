fn is_prime(x : u16) -> bool
{
    if x == 2
    {
        return true;
    }
    if x < 2 || x.is_multiple_of(2)
    {
        return false;
    }
    let mut i : u16 = 3;
    loop 
    {
        if i as u8 == u8::MAX || i * i > x
        {
            return true;
        }
        if x.is_multiple_of(i)
        {
            return false;
        }
        i += 2;
    }
}

fn next_prime(x: u16) -> Option<u16>
{
    (x+1..u16::MAX).into_iter().find(|&i|is_prime(i))
}

fn p1()
{
    let mut i : u16 = 0;
    loop
    {
        i = next_prime(i).unwrap_or(0);
        if i == 0
        {
            break;
        }
        else 
        {
            println!("{i}");   
        }

    }
}

fn checked_addition(x : u32, y : u32) -> u32
{
    if x > u32::MAX - y
    {
        panic!("Overflow for u32 addition: {x} + {y}");
    }

    x + y
}

fn checked_multiplication(x : u32, y : u32) -> u32
{
    if x > u32::MAX / y
    {
        panic!("Overflow for u32 multiplication: {x} * {y}");
    }

    x * y
}

fn p2()
{
    checked_addition(1, 2);
    checked_addition(u32::MAX, 67);

    checked_multiplication(10, 20);
    checked_multiplication(u32::MAX / 10, 11);
}

enum MyError
{
    Overflow
}

fn checked_addition2(x : u32, y : u32) -> Result<u32, MyError>
{
    if x > u32::MAX - y
    {
        Err(MyError::Overflow)
    }
    else
    {
        Ok(x + y)
    }
}

fn checked_multiplication2(x : u32, y : u32) -> Result<u32, MyError>
{
    if x > u32::MAX / y
    {
        Err(MyError::Overflow)
    }
    else
    {
        Ok(x * y)
    }
}

fn p3() -> Result<(), MyError>
{
    checked_addition2(1, 2)?;
    checked_addition2(u32::MAX, 67)?;

    checked_multiplication2(10, 20)?;
    checked_multiplication2(u32::MAX / 10, 11)?;

    Ok(())
}

enum CharErrors
{
    CharNotAscii,
    CharNotDigit,
    CharNotBase16Digit,
    CharNotLetter,
    CharNotPrintable
}

fn to_uppercase (c : char) -> Result<char, CharErrors>
{
    if !c.is_alphabetic()
    {
        Err(CharErrors::CharNotLetter)
    }
    else 
    {
        Ok(c.to_ascii_uppercase())
    }
}

fn to_lowercase (c : char) -> Result<char, CharErrors>
{
    if !c.is_alphabetic()
    {
        Err(CharErrors::CharNotLetter)
    }
    else 
    {
        Ok(c.to_ascii_lowercase())
    }
}

fn print_char (c : char) -> Result<(), CharErrors>
{
    if c.is_ascii_control()
    {
        Err(CharErrors::CharNotPrintable)
    }
    else 
    {
        println!("{c}");
        Ok(())
    }
}

fn char_to_number (c : char) -> Result<u32, CharErrors>
{
    if !c.is_ascii()
    {
        Err(CharErrors::CharNotAscii)
    }
    else if !c.is_ascii_digit()
    {
        Err(CharErrors::CharNotDigit)   
    }
    else 
    {
        Ok(c.to_digit(10).unwrap())
    }
}

fn char_to_number_hex (c : char) -> Result<u32, CharErrors>
{
    if !c.is_ascii()
    {
        Err(CharErrors::CharNotAscii)
    }
    else if !c.is_ascii_hexdigit()
    {
        Err(CharErrors::CharNotBase16Digit)   
    }
    else 
    {
        Ok(c.to_digit(16).unwrap())
    }
}

fn print_error (error : CharErrors)
{
    match error
    {
        CharErrors::CharNotAscii => println!("Caracterul nu este ascii"),
        CharErrors::CharNotDigit => println!("Caracterul nu este cifra"),
        CharErrors::CharNotBase16Digit => println!("Caracterul nu este cifra in baza 16"),
        CharErrors::CharNotLetter => println!("Caracterul nu este litera"),
        CharErrors::CharNotPrintable => println!("Caracterul nu este printabil")
    }
}

fn p5() -> Result<(), CharErrors>
{
    println!("{}", to_uppercase('c')?);
    println!("{}", to_uppercase('1')?);

    println!("{}", to_lowercase('C')?);
    println!("{}", to_lowercase('1')?);

    print_char('J')?;
    print_char('\n')?;
    
    println!("{}", char_to_number('7')?);
    println!("{}", char_to_number('£')?);
    println!("{}", char_to_number('a')?);

    println!("{}", char_to_number_hex('f')?);
    println!("{}", char_to_number_hex('£')?);
    println!("{}", char_to_number_hex('J')?);

    print_error(CharErrors::CharNotAscii);
    print_error(CharErrors::CharNotDigit);
    print_error(CharErrors::CharNotBase16Digit);
    print_error(CharErrors::CharNotLetter);
    print_error(CharErrors::CharNotPrintable);

    Ok(())
}

fn main()
{
    p1();
    println!();
    p2();
    println!();
    if p3().is_err()
    {
        panic!("p3() has error");
    }
    println!();
    if p5().is_err()
    {
        panic!("p5() has error");
    }

}

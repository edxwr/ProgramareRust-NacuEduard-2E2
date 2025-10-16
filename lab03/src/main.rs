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
        if i as u8 >= u8::MAX || i * i > x
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
    for i in x+1..u16::MAX
    {
        if is_prime(i)
        {
            return Some(i);
        }
    }
    return None;
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

fn p3() -> Result<u32, MyError>
{
    checked_addition2(1, 2)?;
    checked_addition2(u32::MAX, 67)?;

    checked_multiplication2(10, 20)?;
    checked_multiplication2(u32::MAX / 10, 11)?;
}

fn main()
{
    p1();
    println!();
    p2();
    println!();
    p3()?;
}

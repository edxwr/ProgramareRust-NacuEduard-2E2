use std::{io, fs};

fn p1() -> Result<(), io::Error>
{
    let s : String = fs::read_to_string("src/example.txt")?;

    let mut longest_bytes_string : String = String::from("");
    let mut longest_chars_string : String = String::from("");

    for line in s.lines()
    {
        if line.len() > longest_bytes_string.len()
        {
            longest_bytes_string = String::from(line);
        }
        if line.chars().count() > longest_chars_string.chars().count()
        {
            longest_chars_string = String::from(line);
        }
    }

    println!("Cea mai lunga linie dupa bytes: {}\nCea mai lunga linie dupa caractere: {}", longest_bytes_string, longest_chars_string);

    Ok(())
}

fn p2(string : String) -> Result<String, String>
{
    let mut new_string : String = String::with_capacity(string.capacity());
    let mut has_error : bool = false;

    for c in string.chars()
    {
        if !c.is_ascii_alphabetic()
        {
            has_error = true;
            break;
        }
        else
        {
            if c.is_ascii_lowercase()
            {
                let index : u8 = c as u8 - 97;
                new_string.push((if index < 13 { index + 13 } else { index - 13 } + 97) as char);
            }
            else
            {
                let index : u8 = c as u8 - 65;
                new_string.push((if index < 13 { index + 13 } else { index - 13 } + 65) as char);
            }
        }
    }

    if has_error
    {
        Err(String::from("Stringul contine un caracter non-ascii-alphabetic!"))
    }
    else
    {
        Ok(new_string)
    }

}

fn p3() -> Result<String, io::Error>
{
    let string : String = fs::read_to_string("src/p3file.txt")?;
    let mut new_string : String = String::with_capacity(string.capacity());

    for word in string.split(' ')
    {
        match word
        {
            "pt" | "ptr" => new_string.push_str("pentru"),
            "dl" => new_string.push_str("domnul"),
            "dna" => new_string.push_str( "doamna"),
            _ => new_string.push_str(word)
        }

        new_string.push(' ');
    }

    Ok(new_string)
}
fn main()
{
    match p1()
    {
        Ok(()) => (),
        Err(e) => println!("{}", e)
    }
    match p2(String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"))
    {
        Ok(string) => println!("Stringul dupa ROT13:\t{}", string),
        Err(e) => println!("{}", e)
    }
    match p3()
    {
        Ok(string) => println!("Stringul dupa abrevieri:\t{}", string),
        Err(e) => println!("{}", e)
    }
}

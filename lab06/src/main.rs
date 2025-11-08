use std::fs;

use rusqlite::Connection;

trait Executable
{
    fn get_name(&self) -> String;
    fn exec(&mut self, slice : &str);
}

impl Executable for PingCommand
{
    fn get_name(&self) -> String
    {
        String::from("ping")
    }
    fn exec(&mut self, slice : &str)
    {
        let _ = slice.chars();
        println!("pong!")
    }
}

impl Executable for CountCommand
{
    fn get_name(&self) -> String
    {
        String::from("count")
    }
    fn exec(&mut self, slice : &str)
    {
        println!("counted {} args", slice.split_whitespace().count())
    }
}

impl Executable for TimesCommand
{
    fn get_name(&self) -> String
    {
        String::from("times")
    }
    fn exec(&mut self, slice : &str)
    {
        let _ = slice.chars();
        self.count += 1;
        println!("Times command called:\t{}", self.count);
    }
}

impl Executable for DrawHeartCommand
{
    fn get_name(&self) -> String
    {
        String::from("drawheart")
    }
    fn exec(&mut self, slice : &str)
    {
        let _ = slice.chars();
        println!(" ❤❤   ❤❤");
        println!(" ❤❤❤❤❤❤❤");
        println!(" ❤❤❤❤❤❤❤");
        println!("  ❤❤❤❤❤");
        println!("   ❤❤❤");
        println!("    ❤");
    }
}

struct PingCommand
{

}

struct CountCommand
{

}

struct TimesCommand
{
    count : u32
}

struct DrawHeartCommand
{

}

impl Terminal
{
    fn new() -> Terminal
    {
        Terminal { commands : Vec::with_capacity(3) }
    }
    fn register(&mut self, command : Box::<dyn Executable>)
    {
        self.commands.push(command);
    }
    fn run(&mut self, file_path : &str)
    {
        let result_input_commands  = fs::read_to_string(file_path);
        let input_commands : String;
        match result_input_commands
        {
            Ok(cmds) =>
            {
                input_commands = cmds;
                'start: for line in input_commands.lines()
                {
                    let mut words = line.split_whitespace();
                    if words.clone().count() == 0
                    {
                        println!("Empty command!");
                        continue;
                    }
                    match words.next()
                    {
                        Some(cmd) =>
                        {
                            if "stop" == cmd
                            {
                                break;
                            }
                            else if "stop".eq_ignore_ascii_case(cmd)
                            {
                                println!("Command was {}, did you mean stop?", cmd);
                                continue;
                            }
                            for command in &mut self.commands
                            {
                                if command.get_name() == cmd
                                {
                                    command.exec(&line[cmd.len()..]);
                                    continue 'start;
                                }
                                else if command.get_name().eq_ignore_ascii_case(cmd)
                                {
                                    println!("Command was {}, did you mean {}?", cmd, command.get_name());
                                    continue 'start;
                                }
                            }
                            println!("Command not registered!");
                        }
                        None => println!("Eroare la linie din fisier!")
                    }
                }
            },
            Err(e) => println!("Eroare la citire din fisier:\t{}", e)
        }
    }
}

struct Terminal
{
    commands : Vec<Box::<dyn Executable>>
}

fn p1()
{
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(DrawHeartCommand {}));

    terminal.run("commands.txt");
}

impl Executable for BookmarkCommand
{
    fn get_name(&self) -> String
    {
        String::from("bk")
    }
    fn exec(&mut self, slice : &str)
    {
        let conn = Connection::open("bookmarks.db");
        let mut items = slice.split_whitespace();
        if let (Some(cmd), Ok(connection)) = (items.next(), conn)
        {
            let create = r"
            create table if not exists bookmarks (
                name text    not null,
                url  text not null
            );
            ";
            if connection.execute(create, ()).is_err()
            {
                println!("Eroare la execute");
                return;
            }
            match cmd
            {
                "add" =>
                {
                    if items.clone().count() != 2
                    {
                        println!("Too little/many arguments for bk add.\tUsage:\tbk add <name> <url>");
                        return;
                    }
                    let arg1 = items.next();
                    let arg2 = items.next();
                    if let (Some(name), Some(url)) = (arg1, arg2)
                    {
                        if connection.execute("insert into bookmarks (name, url) values (?1, ?2);", (name, url)).is_err()
                        {
                            println!("Eroare la execute\t");
                        }
                    }
                    else
                    {
                        println!("Eroare la add");
                    }
                },
                "search" =>
                {
                    if items.clone().count() != 1
                    {
                        println!("Too little/many arguments for bk add.\tUsage:\tbk search <name>");
                        return;
                    }

                    let arg = items.next();

                    struct Bookmark
                    {
                        name : String,
                        url : String
                    }

                    let mut sql_command : String = String::from("select distinct * from bookmarks where name like '");
                    if let Some(name) = arg
                    {
                        sql_command.push_str(name);
                    }
                    else
                    {
                        println!("Eroare la search");
                    }
                    sql_command.push_str("%'");

                    let stmt_result = connection.prepare(sql_command.as_str());

                    if let Ok(mut stmt) = stmt_result
                    {
                        let bookmark_iter_result = stmt.query_map([], |row| {
                            Ok(Bookmark {
                                name: row.get("name")?,
                                url: row.get("url")?
                            })
                        });

                        if let Ok(bookmark_iter) = bookmark_iter_result
                        {
                            for i in bookmark_iter
                            {
                                if let Ok(i) = i
                                {
                                    println!("name={}, url={}", i.name, i.url);
                                }
                                else
                                {
                                    println!("Eroare la bookmark_iter");
                                    return;
                                }
                            }
                        }
                        else
                        {
                            println!("Eroare la query_map");
                        }
                    }
                    else
                    {
                        println!("Eroare la prepare");
                    }
                }
                _ =>
                {
                    println!("Unknown bk first argument.\tUsage:\tbk add/search");
                }
            }
        }
        else
        {
            println!("Eroare la executare BookmarkCommand");
        }
    }
}

struct BookmarkCommand
{

}

fn p2()
{
    let mut terminal = Terminal::new();

    terminal.register(Box::new(BookmarkCommand {}));

    terminal.run("bk_commands.txt");
}
fn main()
{
    p1();
    p2();
}
use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};
use std::fmt::Write;
use std::str;
use std::fmt;
/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}
const ALARM:u8 =7 as u8;
const ENTER:u8 =13 as u8;
const NEWLINE:u8=10 as u8;
const BACKSPACE:u8=8 as u8;
const DELETE:u8=127 as u8;
/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    let mut console=CONSOLE.lock();
    let mut storage = [0u8; 1024];
    let mut cmd=StackVec::new(&mut storage);
    'outer:loop {
        cmd.truncate(0);        
        let mut num=0;
        console.write_str(prefix);
        'inner:loop {
            let b=console.read_byte();            
            if num>511{
                if b!=ENTER&&b!=NEWLINE&&b!=BACKSPACE&&b!=DELETE{
                    console.write_byte(ALARM);
                    continue 'inner;
                }
            }
            match b {
                BACKSPACE|DELETE => {
                    if num==0{
                        console.write_byte(ALARM);
                    }
                    else{
                        num-=1;
                        console.write_byte(BACKSPACE);
                        console.write_byte(b' ');
                        console.write_byte(BACKSPACE);
                        cmd.pop();
                    }
                },
                ENTER => {
                        console.write_byte(ENTER);
                        console.write_byte(NEWLINE);
                        break 'inner;
                },
                _ =>{
                    num+=1;
                    console.write_byte(b);
                    cmd.push(b);
                }
            }
        }
        let res= str::from_utf8(cmd.as_slice());
        let cmd_str=match res {
            Ok(s) => s,
            Err(e) =>"",
        };
        let mut buf=&mut ["";100];
        let mut shell=Command::parse(cmd_str,buf);
        match shell {
            Ok(command) => {
                    let path=command.path();
                    // if path=="echo"{
                        // for s in command.args[1..].iter(){
                        //         console.write_str(s);
                        //         console.write_byte(b' ');
                        //     }
                    // }else{
                    //         console.write_str("Unknown command");

                    // }
                    match path {
                        "echo" => {
                            for s in command.args[1..].iter(){
                                console.write_str(s);
                                console.write_byte(b' ');
                            }
                        },
                        _ => {
                            console.write_str("Unknown command");
                            
                        },
                    }
                    console.write_byte(ENTER);
                    console.write_byte(NEWLINE);
            }
            Err(Empty) => {},
            Err(TooManyArgs)=>{
                console.write_str("Too many args");
                console.write_byte(ENTER);
                console.write_byte(NEWLINE);
            }
        }
        
    }

}

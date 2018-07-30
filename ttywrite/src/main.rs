extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use] extern crate structopt_derive;

use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};
use xmodem::{Xmodem, Progress};

mod parsers;

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn main() {
    use serial::prelude::*;
    use std::fs::File;
    use std::io::{self, BufReader, BufRead,Write,Read,Cursor};
    let opt = Opt::from_args();
    let mut serial = serial::open(&opt.tty_path).expect("path points to invalid TTY");
    serial.reconfigure(&|settings| {
        settings.set_baud_rate(opt.baud_rate);
        settings.set_char_size(opt.char_width);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    });
    let mut l:usize=0;
    serial::SerialPort::set_timeout(&mut serial,Duration::new(opt.timeout,0)).expect("set time fail");
    match (opt.input,opt.raw) {
        (None,true) => {
                        let input=io::stdin();
                        let mut f=BufReader::new(input);
                        let mut vec=vec![];
                        io::copy(&mut f,&mut vec).expect("copy fail");
                        serial.write(&vec[..]).expect("write fail");
                        l=vec.len();
        }
        (Some(buf),true) => {
                        let input=File::open(buf.as_path()).expect("can't open file");
                        let mut f=BufReader::new(input);
                        let mut vec=vec![];
                        io::copy(&mut f,&mut vec).expect("copy fail");                        
                        serial.write(&vec[..]).expect("write fail");
                        l=vec.len();
        }
        (None,false)    =>{
                        let input=io::stdin();
                        let mut f=BufReader::new(input);
                        let mut vec=vec![];
                        io::copy(&mut f,&mut vec).expect("copy fail");                        
                        l=Xmodem::transmit(&vec[..],serial).expect("transmit fail");
        }
        (Some(buf),false)=>{            
                        let input=File::open(buf.as_path()).expect("can't open file");
                        let mut f=BufReader::new(input);
                        let mut vec=vec![];
                        io::copy(&mut f,&mut vec).expect("copy fail");                        
                        l=Xmodem::transmit(&vec[..],serial).expect("transmit fail");
        }
    }
    print!("wrote {} bytes to {:?}\n",l,opt.tty_path);
}

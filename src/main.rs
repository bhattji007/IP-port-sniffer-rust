use std::{env, io::{self, Write}, net::{IpAddr, TcpStream}, process, str::FromStr, sync::mpsc::{channel, Sender}, thread};

const MAX:u16=65535;
struct Arguments{
    flag:String,
    ipaddrs: IpAddr,
    threads: u16
}


    impl Arguments{    //takes string arguments and resturns the struct argument or error in string form
        fn new(args:&[String])->Result<Arguments,&'static str>{
            if args.len()<2{
                return Err("Not enough Arguments (⁠╥ ⁠﹏ ⁠╥) !!");
            }
            else if args.len()>4{
                return Err("Too many  Arguments (⁠╥ ⁠﹏ ⁠╥) !!");
            }
            let f =args[1].clone();
            if let Ok(ipaddrs)=IpAddr::from_str(&f){   //converting the next arg in cmdline considering its an ipaddress to string 
                return Ok(Arguments{flag:String::from(""),ipaddrs,threads:4});
            }
            else{
                let flag=args[1].clone();
                if flag.contains("-h")|| flag.contains("-help") && args.len()==2{
                    println!("Usage: -j to seleect how many threads you want 
                    \r\n    -h or -help to show this help message happy coding (⁠≧ ⁠▽ ⁠≦⁠) !!");
                    return Err("Help (⁠╥ ⁠﹏ ⁠╥) !!");
                }else if flag.contains("-h")|| flag.contains("-help"){
                    return Err("Too many arguments (⁠╥ ⁠﹏ ⁠╥) !!");
                }else if flag.contains("-threads"){
                    let ipaddrs=match IpAddr::from_str(&args[3]){
                        Ok(s)=>s,
                        Err(_)=> return Err("not a valid Ip Address; must be a IPV4 or IPV6 (⁠╥ ⁠﹏ ⁠╥⁠) !!")
                    };
                    let threads= match args[2].parse::<u16>(){
                        Ok(s)=>s,
                        Err(_)=> return Err("Can't parse into numerical value (⁠╥ ⁠﹏ ⁠╥⁠) !!")
                    };
                    return Ok(Arguments{threads,flag,ipaddrs});
                }
                else{
                    return Err("Inavlid Syntax (⁠╥ ⁠﹏ ⁠╥) !");
                }
            }
        }
    }

    fn scan (tx:Sender<u16>,start_port:u16,addr:IpAddr,num_threads:u16){
        let mut port :u16=start_port+1;
        loop{
            match TcpStream::connect((addr,port)){
                Ok(_)=>{
                    println!("(⁠≧ ⁠▽ ⁠≦⁠)");
                    io::stdout().flush().unwrap();
                    tx.send(port).unwrap();
                },
                Err(_)=>{}
            };
            if (MAX-port)<=num_threads{
                break;
            }
            port+=num_threads;
            println!("hello")
        }
    }



fn main() {
    let args:Vec<String>=env::args().collect();   // collect the incoming args from cmdline as string vector
    let program =args[0].clone();
    let arguments =Arguments::new(&args).unwrap_or_else(
        |err|{
            if err.contains("Help (⁠╥ ⁠﹏ ⁠╥) !!"){
                process::exit(0);
            }   
            else{
                eprintln!("{} problem missing arguments :{} (⁠╥ ⁠﹏ ⁠╥) ",program, err);
                process::exit(0);
            }

        }
        
    );
    let num_threads= arguments.threads;
    let addr=arguments.ipaddrs;
    let (tx,rx)=channel();
    for i in 0..num_threads{
        let tx=tx.clone();
        thread::spawn(move || {
            scan(tx,i,addr,num_threads);
        });
    }
    let mut out= vec![];
    drop(tx);
    for p in rx{
        out.push(p);
    }
    println!("__");
    out.sort();
    for v in out{
        println!("{} is open (⁠≧⁠▽⁠≦⁠)",v);
    }


}

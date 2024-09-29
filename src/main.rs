use std::env;

struct Arguments{
    tag:String,
    threads: u16,
    ipaddr: String
}

impl Arguments {

    fn new(args: Vec<String>) -> Result<Arguments, String>{

      if args.len() > 4 {
        return Err("Error: Too many arguments. 
                    \r\n Example input: -h 10 168.0.0.1".to_string());
      } else if args.len() < 2 {
        return Err("Error: Too few arguments. 
                    \r\n Example input: -h 10 168.0.0.1".to_string());          
      } else {

        if args.len() == 2{
          let tag = args[1].clone();
          if tag.contains("-h") || tag.contains("-help"){
            return Ok(Arguments{tag, threads: 0, ipaddr: "".to_string()});
          } else {
            return Err("Error: Invalid syntax. 
                        \r\n Example input: -h 10".to_string());
        }
        } else {
            return Err("Error: Invalid syntax. 
                        \r\n Example input: -h 10".to_string());
        }

      }
    }    
  }

fn main() {
  let args:Vec<String> = env::args().collect();
  let arguments = Arguments::new(args).unwrap_or_else(|err|{
    println!("{}", err);
    std::process::exit(1);
  });

  println!("Tag: {}", arguments.tag);
  println!("Threads: {}", arguments.threads);
  println!("IP Address: {}", arguments.ipaddr);
}

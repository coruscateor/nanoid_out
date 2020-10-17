//extern crate nanoid;
use nanoid::nanoid;
use std::io; //::{self, Read};
fn main() {

    let mut exit: bool = false;

    loop {

        let id = nanoid!();

        let mut buffer = String::new();

        let stdin = io::stdin();
        //let mut handle = stdin.lock();

        println!("{}", id);

        //handle.read_to_string(&mut buffer).unwrap(); //?

        match stdin.read_line(&mut buffer)
        {

            Ok(_) => {

                /*
                if buffer == "q" || buffer == "Q"
                {

                    exit = true;

                }
                */

                //println!("Ok");

                //let buf_str = buffer.as_str();
                
                //if buf_str == "q" //|| buf_str == "Q"

                //println!("{}", buffer);

                //let q: &str = "q";

                //println!("{}", buffer.eq(q));

                //the input buffer needs to be trimmed for some reason

                let trimmed_buffer = buffer.trim();

                //println!("{}", trimmed_buffer == "q");

                if trimmed_buffer == "q" || trimmed_buffer == "Q"
                //if buffer.eq(q)
                {

                    exit = true;

                }

                /*
                match buf_str
                {

                    "q" | "Q" => { exit = true; },
                    _ => { }

                }
                */
            },
            Err(e) => {

                println!("{}", e);

            }

        }

        if exit
        {

            println!("Exiting");

            break;

        }

    }

}

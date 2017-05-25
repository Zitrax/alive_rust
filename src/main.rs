use std::env::args;
use std::thread;
use std::time::Duration;

extern crate oping;
use oping::Ping;

// Macro to immediately kill the program on error
macro_rules! try {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => {
            println!("ERROR: {}", err);
            std::process::exit(1);
        }
    });
}

// Just trying rust...
//
// Using oping crate to ping a server specified as argument to main
//
fn main() {
    if args().len() < 2 {
        println!("No arguments. Aborting!");
        std::process::exit(1);
    }

    loop {
        let mut ping = Ping::new();
        try!(ping.set_timeout(0.5));

        for host in args().skip(1) {
            try!(ping.add_host(&host));
        }

        let res = try!(ping.send());

        // Clear screen
        print!("{}[2J", 27 as char);
        for r in res {
            println!("{}: {} ms", r.hostname, r.latency_ms);
        }
        thread::sleep(Duration::from_millis(500));
    }
}

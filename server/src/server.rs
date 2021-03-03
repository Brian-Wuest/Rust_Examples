use std::io::Read;
use std::net::TcpListener;
use std::convert::TryFrom;
// Use "crate" instead of "super" to start at the root of the project to find modules/classes.
use crate::http::Request;

// Everything in a module is private by default.
// use 'pub' to make it public
// Each file is it's own module.
// Contains the data associated with a struct (class).
pub struct Server {
    addr: String,
}

// In order to not have array parameter with hard-set sizes they need to be prefaced with &
// This makes them a reference and the compiler knows how large references are since it's just a pointer.
// This also makes the array a "slice" which is similar to a "string-slice"
/* fn arr(a: &[u8]) {

} */

/*
   Contains the implementation logic for the struct.
   Implementations are methods or associated functions.
   Methods take special parameter called 'self'
       Methods are "instance" functions
       'self' is similar to 'this' in C#

   Associated functions are on the "Type".
       These are like "static methods" in C#
*/
impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr: addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        // Special keyword for infinite loops is just "loop".
        // Loops can be labeled for breaks from inner loops.
        // Loops are labeled as: 'label_name: loop
        // The beginning single quote is important
        loop {
            // Match is a special keyword in rust which allows us to match on enum variants
            // with some conditions.
            // Need to cover every variant of the enum or compilation will fail.
            // Match doesn't have to be used just on function results.
            // It can be used like Switch-Case statements in other languages.
            match listener.accept() {
                // Do specific logic when the result is "okay".
                // Can unrap result tuple into separate variables like we can with regular results.
                // use _ in the match parameter to ignore the parameter.
                // Can do the same thing for tuple unwraps.
                // Can also use _ as part of a Match pattern to handle all un-handled enum variants.
                // Used as a catch-all for things we don't want/need to write code for.
                // Similar to "default" in switch-case statements in C#
                // Example: Ok(_)
                // Example: Ok((stream, _))
                // Example: _ =>
                Ok((mut stream, addr)) => {
                    // Declares an array filled with zeroes to the specified amount of indexes.
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // Print the resulting request; using _lossy means that the function could never fail
                            // This allows us to always print the requested data.
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // Explicity convert to byte slice using "as" keyword
                            // Request::try_from(&buffer as &[u8]);

                            // Convert the buffer to a byte slice containing the entire array.
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                },
                                Err(e) => {
                                    println!("Failed to parse a request: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from the connection: {}", e);
                        }
                    }
                }
                // Do specific logic when the result is "error"
                Err(e) => {
                    println!("Failed to establish a connection: {}", e);
                }
            }
            /*
            match "abcd" {
                "abcd" => println!(),
                // The pipe here means it can match against "a" or "b"
                "a" | "b" => {},
                _ => {}
            }
            */
        }
    }
}

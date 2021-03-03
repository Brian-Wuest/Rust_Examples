#![allow(dead_code)]

use server::Server;
use http::Method;
use http::Request;

// Finds the server module in the server.rs file.
// the server module in the server.rs file is implicit based on the file name.
mod server;
mod http;

fn main() {
    // String != String literals
    /*
        Strings are mutable and can be shrunk/expanded as needed.
        String Slices (like String Literals) are immutable.
        Strings in Rust are UTF-8 encoded.
        Don't use index-based slicing since a character can take up more than 1 byte of data.
        Doing &string[10..] tells the process to take everything after the 10th byte, not the 10th character.
    */
    /*
        let string_slice = &string[10..];
        let string_borrow: &str = &string;
        let string_literal = "1234";

        dbg!(&string);
        dbg!(&string_slice);
        dbg!(&string_borrow);
        dbg!(&string_literal);
    */

    let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

/*
GET /users?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

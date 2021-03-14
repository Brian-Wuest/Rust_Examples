#![allow(dead_code)]

use server::Server;
use http::Request;
use website_handler::WebsiteHandler;
use std::env;

// Finds the server module in the server.rs file.
// the server module in the server.rs file is implicit based on the file name.
mod server;
mod http;
mod website_handler;

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

    // The default path is the current directory (determined by the cargo.toml file)
    // plus "/public"
    let default_path = format!("{}\\public", env!("CARGO_MANIFEST_DIR"));

    // If the environment variable does not exist; use the default path instead.
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

    /* let get = Method::GET;
    let delete = Method::DELETE;
    let post = Method::POST;
    let put = Method::PUT; */

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebsiteHandler::new(public_path));
}

/*
GET /users?id=10 HTTP/1.1\r\n
HEADERS \r\n
BODY
*/

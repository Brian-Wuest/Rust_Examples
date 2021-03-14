use crate::http::Method;
use crate::http::Response;
use crate::http::StatusCode;
use crate::server::Handler;
use crate::Request;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        // Resolve the path provided and check the path against our public path.
        // If they match then this is a valid request.
        // Otherwise this might be an attack.
        match fs::canonicalize(path) {
            Ok(path) => {
                // canonicalize can put add the "Extended Length path" prefix to the string when on windows.
                // Make sure to remove this otherwise the strings cannot be compared correctly.
                let cleaned_path = path.to_string_lossy().replace("\\\\?\\", "");

                if cleaned_path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            },
            Err(_) => None
        }

        // This "ok" method is on the "Result" type which converts the 
        // "Ok" result into a "Some" option and an "Error" result into a "None" option.
        //fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

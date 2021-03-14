use crate::http::status_code::StatusCode;
use std::io::{Write, Result as IoResult};

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // The "stream" parameter is a Static Dispatch of the "Write" trait.
    // Static Dispatch is better vs Dynamic Dispatch because Dyanamic Dispatch
    // Happens during run-time vs Static Dispatch occuring during Compile time.
    // When Dynamic Dispatch occurs it looks up the proper implementation using an in-memory V-Table
    // This happens before it can call the associated function and then it can go to the appropriate function location and call it.
    // Static Dispatch is preferred over Dynamic Dispatch.
    // Static Dispatch increases compile time and Binary File Size.
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}


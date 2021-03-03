use super::method::Method;
use super::{QueryString};
use crate::http::parse_error::ParseError;
use std::convert::TryFrom;
use std::str;

/*
    Use the 'buf syntax to declare a lifetime for the struct.
    This will indicate to rust how long of a life memory allocations can be.
    lifetimes are named by the engineer and are short (usually 1 character or something useful).

    They are decclared as part of a 'generic' definition on the struct
    Implementations also need this named lifetime as part of the 'impl' keyword as shown below.
*/
// This special syntax tells the compiler that this class can be formatted in debug calls.
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    // Using Option<string> here allows us to define that the value can be null (or no value)
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

/**
 * Implements the "TryFrom" trait (interface in C#) for the custom "Request" Type.
 * For the "TryFrom" trait; an implicit "TryInto" trait is also implemented.
 * This means that two-way conversion is initiated by only implementing a single trait.
 */
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        // The "?" at the end of this "or" means that if there is an error it will be returned
        // Otherwise it will un-wrap the Ok value from the result and return it.
        // This make for much nicer code since we would otherwise have to create a "match" block.
        // let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?;

        // This line is even shorter than above since we have implemented the "From" trait for ParseError.
        // We don't need to use the "or" function on the from_utf8 method because of this implementation.
        let request = str::from_utf8(buf)?;

        // Overwriting "request" variable here (variable shadowing).
        // Using "ok_or" here turns the "Option" type to a "Result" type and allows us to use the short-hand ? for handling the error.
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (proto, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if proto != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // use "if let" to initialize variable, usefull when you don't care about the "none" part of an Option.
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

/**
 * Returns a tuple containing the first word found in the passed-in string.
 * The second part of the tuple is the remaining part of the string.
 */
fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // Using .enumerate on this gives the index of the item as well as it's value!!!!
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' || c == '\n' {
            let mut start: &str = &request[..i];
            let mut end: &str = &request[i + 1..];
            start = start.trim();
            end = end.trim();

            return Some((start, end));
        }
    }

    None
}

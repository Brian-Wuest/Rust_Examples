use std::fmt::{Display, Formatter, Result as FmtResult};

// Copy derived trait requires Clone derived type.
// Note: it is also good practice to include Debug Derived trait on all enum/structs.
#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            // Multiple ways of returning values from "matches"
            // Use what is most readable expected for standards.
            Self::Ok => {
                return "Ok";
            }
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // In this case when converting the StatusCode to a u16 we need to "de-reference" our reference variable
        // Reference variables of the "&" and are just pointers to the actual value in memory.
        // This means we will instead "copy" the information the reference is pointing to.
        // In order to do this we need the "Copy" or "Clone" trait on the enum.
        write!(f, "{}", *self as u16)
    }
}

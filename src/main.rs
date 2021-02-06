use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::read_to_string;

// This is a custom error type.
// It enables callers that receive this kind of error
// to handle different error causes differently.
// These must implement the Error trait which requires
// implementing the Debug and Display traits.
#[derive(Debug)]
pub enum GetDogsError {
    BadFile(std::io::Error),
    BadJson(serde_json::error::Error),
}

// Make the variants of this enum directly available.
use GetDogsError::*;

// All of the Error trait methods have default implementations, so
// no body is required here, but we will implement the source method.
impl Error for GetDogsError {
    // Returns the wrapped error, if any.
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            // The wrapped error type is implicitly cast to the trait object
            // type &Error because it implements the Error trait.
            BadFile(ref e) => Some(e),
            BadJson(ref e) => Some(e),
        }
    }
}

impl std::fmt::Display for GetDogsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BadFile(ref e) => write!(f, "bad file: {}", e),
            BadJson(ref e) => write!(f, "bad JSON: {}", e),
        }
    }
}

// The "From" trait converts values of one type to another.
// Having the following implementations enables
// using the ? operator in the get_dogs3 function below.
impl From<std::io::Error> for GetDogsError {
    fn from(other: std::io::Error) -> Self {
        BadFile(other)
    }
}
impl From<serde_json::error::Error> for GetDogsError {
    fn from(other: serde_json::error::Error) -> Self {
        BadJson(other)
    }
}

// This struct can be deserialized from JSON and serialized to JSON.
#[derive(Deserialize, Serialize, Debug)]
struct Dog {
    name: String,
    breed: String,
}

// Let's look at three versions of a function that
// reads a JSON file describing dogs and parses it
// to create a vector of Dog instances.

// With this version callers cannot easily distinguish between
// the two types of errors that can occur,
// std::io:Error from failing to read the file and
// serde_json::error::Error from failing to parse the JSON.
// This approach is fine when callers only need to
// know if an error occurred and print an error message.
fn get_dogs1(file_path: &str) -> Result<Vec<Dog>, Box<dyn Error>> {
    let json = read_to_string(file_path)?;
    let dogs: Vec<Dog> = serde_json::from_str(&json)?;
    Ok(dogs)
}

// If we have many functions with this same return type,
// Result<some-type, GetDogsError> {
// we can reduce the repetition by defining a type alias.
pub type MyResult<T> = std::result::Result<T, GetDogsError>;

// With this version callers can distinguish between the
// two types of errors by matching on the GetDogsError variants.
fn get_dogs2(file_path: &str) -> MyResult<Vec<Dog>> {
    match read_to_string(file_path) {
        Ok(json) => match serde_json::from_str(&json) {
            Ok(dogs) => Ok(dogs),
            Err(e) => Err(BadJson(e)),
        },
        Err(e) => Err(BadFile(e)),
    }
}

// This version takes advantage of the fact that
// GetDogsError implements the From trait for
// each of the kinds of errors that can occur.
// This enables using the ? operator because errors of those
// types will automatically be converted to the GetDogsError type.
fn get_dogs3(file_path: &str) -> MyResult<Vec<Dog>> {
    let json = read_to_string(file_path)?;
    let dogs: Vec<Dog> = serde_json::from_str(&json)?;
    Ok(dogs)
}

// If the main function has this return type, it can use the ? operator.
//fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    let file_path = "./dogs.json";

    /*
    // With the first approach we can easily detect that an error has occurred.
    if let Ok(dogs) = get_dogs1(file_path) {
        dbg!(dogs);
    } else {
        eprintln!("failed to get dogs, but don't know why");
    }
    */

    /*
    // But handling different kinds of errors differently is messy.
    match get_dogs1(file_path) {
        Ok(dogs) => println!("{:?}", dogs),
        Err(e) => {
            if let Some(e) = e.downcast_ref::<std::io::Error>() {
                eprintln!("bad file: {:?}", e);
            } else if let Some(e) = e.downcast_ref::<serde_json::error::Error>() {
                eprintln!("bad json {:?}", e);
            } else {
                eprintln!("some other kind of error");
            }
        }
    }
    */

    // With the second and third approaches it is much easier
    // to handle different kinds of errors differently.
    //match get_dogs2(file_path) {
    match get_dogs3(file_path) {
        Ok(dogs) => println!("{:?}", dogs),
        Err(BadFile(e)) => eprintln!("bad file: {}", e),
        Err(BadJson(e)) => eprintln!("bad json: {}", e),
    }
}

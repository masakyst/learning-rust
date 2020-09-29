extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|req: &mut Request| {
        Ok(Response::with((status::Ok, "Hello World!")))
    }).http("127.0.0.1:3000").unwrap();
}

use futures::{future, Future};
use std::io;
use std::sync::{Arc, Mutex};
use tokio_service::Service;

mod codec;
mod protocol;
mod request_parser;

pub use self::protocol::LineProto;
pub use self::codec::LineCodec;
use DB;
use DBCommand;

pub struct Server<T: DB> {
    db: Arc<Mutex<T>>,
}

impl<T: DB> Server<T> {
    pub fn new(db: Arc<Mutex<T>>) -> Server<T> {
        Server { db }
    }
}

impl<T: DB> Service for Server<T> {
    // These types must match the corresponding protocol types:
    type Request = DBCommand;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
        let mut db = self.db.lock().unwrap();
        let response = db.process_command(req);
        Box::new(future::ok(response.unwrap().unwrap_or("None".to_string())))
    }
}

extern crate num_cpus;
extern crate tokio_minidb;
extern crate tokio_proto;

use tokio_minidb::{InMemoryDB, DB};
use tokio_minidb::server::{LineProto, Server};
use tokio_proto::TcpServer;

fn main() {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    // Instantiate DB
    let db_handle = InMemoryDB::new().unwrap().handle();

    println!("Listening on port 12345");

    // The builder requires a protocol and an address
    let mut srv = TcpServer::new(LineProto, addr);
    srv.threads(num_cpus::get());
    srv.serve(move || Ok(Server::new(db_handle.clone())))
}

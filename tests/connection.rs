extern crate xmpp;

use xmpp::Server;

#[test]
fn server_connect() {
    let server_address = env!("SERVER_ADDRESS");
    let server = Server::new(server_address);
    server.connect().unwrap();
}

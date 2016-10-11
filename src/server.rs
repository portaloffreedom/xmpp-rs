use std::io;

use connection::Connection;


pub struct Server {
    addr: String, //SocketAddr,
}

impl Server {
    pub fn new(addr: &str) -> Server {
        Server {
            addr: addr.to_string(),
        }
    }

    pub fn connect(&self) -> Result<Connection, io::Error> {
        let mut conn = try!(Connection::create(self.addr.as_str()));

        let from_id = "user@test.com"; // MyJID.GetBareJID()
        let to_id = "test.com"; // MyJID.GetDomain()

        try!(conn.greatings(from_id, to_id));

        Ok(conn)
    }
}

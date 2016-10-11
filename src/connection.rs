use std::net::TcpStream;
use std::net::ToSocketAddrs;
use std::io;

use std::io::Write;
use std::io::Read;

pub struct Connection {
    conn: TcpStream,
}


impl Connection {
    pub fn create<Addr: ToSocketAddrs>(addr: Addr) -> Result<Connection, io::Error> {
        Ok(Connection {
            conn: try!(TcpStream::connect(addr))
        })
    }

    pub fn greatings(&mut self, from_id: &str, to_id: &str) -> Result<(), io::Error> {

        use xml::writer::XmlEvent;
        use xml::writer::EmitterConfig;

        {
            let mut w = EmitterConfig::new().write_document_declaration(true).create_writer(&mut self.conn);

            w.write(XmlEvent::start_element("stream:stream")
                        .ns("stream", "http://etherx.jabber.org/streams")
                        .attr("from", &from_id)
                        .attr("to", &to_id)
                        .attr("xml:lang", "en")
                        .attr("xmlns", "jabber:client")
            ).unwrap();
            //w.write("hello world").unwrap();
            w.write(XmlEvent::end_element()).unwrap();
        }

        //self.conn.write(open_stream.as_bytes()).unwrap();
        //println!("question: {}", open_stream);
        self.conn.flush().unwrap();

        use std::str;

        let mut response = vec![0; 1024];
        let size = self.conn.read(&mut response).unwrap();
        let response_string = str::from_utf8(&response).unwrap();
        println!("reponse: {}", response_string);
        Ok(())
    }
}

extern crate xmpp;
extern crate xml;

use std::str;

use xml::writer::EmitterConfig;

#[test]
fn writing_simple() {
    use xml::writer::XmlEvent;

    let mut b = Vec::new();
    let from_id = env!("USER_ID"); // MyJID.GetBareJID()
    let to_id = env!("SERVER_DOMAIN"); // MyJID.GetDomain()

    {
        let mut w = EmitterConfig::new().write_document_declaration(true).create_writer(&mut b);

        w.write(XmlEvent::start_element("stream:stream")
            .ns("stream", "http://etherx.jabber.org/streams")
            .attr("from", &from_id)
            .attr("to", &to_id)
            .attr("xml:lang", "en")
            .attr("xmlns", "jabber:client")
//            .attr
        ).unwrap();
        //w.write("hello world").unwrap();
        //w.write(XmlEvent::end_element()).unwrap();
    }

    let string = str::from_utf8(&b).unwrap();

    println!("{}", string);

//    assert_eq!(
//        string,
//        r#"<h:hello xmlns:h="urn:hello-world">hello world</h:hello>"#
//    );
}

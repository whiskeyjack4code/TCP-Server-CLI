use core::fmt;

pub struct Connection {
    server_name:  String,
    port:        String,
}

pub fn connect(name: String, port: String) -> Connection {
    let c = Connection {
        server_name: name.to_string(),
        port: port.to_string(),
    };

    c
}
impl fmt::Display for Connection {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.server_name, self.port)
    }
}

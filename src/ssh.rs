use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;

pub fn new_connection_private_key(address: String, port: String, username: String, key: String) {
    let tcp = TcpStream::connect("").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    let auth_result = sess.userauth_pubkey_file(&username, None, &Path::new(&key), None);
    println!("{:?}", auth_result);
}

pub fn new_connection_password(address: String, port: String, username: String, password: String) {
    let tcp = TcpStream::connect("").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    let auth_result = sess.userauth_password(&username, &password);
    println!("{:?}", auth_result);
}

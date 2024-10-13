use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::str::from_utf8;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub fn handle_session(sess: Session, tx: Sender<String>) {
    let mut channel = sess.channel_session().unwrap();
    let mut mode = ssh2::PtyModes::new();
    mode.set_character(ssh2::PtyModeOpcode::VINTR, Some(3 as char));
    channel
        .request_pty("ansi", Some(mode), Some((500, 500, 500, 500)))
        .unwrap();
    channel.shell().unwrap();
    channel.write_all(b"pwd").unwrap();
    println!("Wrote, entering loop");
    loop {
        let mut buf = [0u8; 1024];
        match channel.read(&mut buf) {
            Ok(0) => break,
            Ok(c) => {
                let slice = &buf[0..c];
                match std::str::from_utf8(slice) {
                    Ok(s) => {
                        print!("{}", s);
                        tx.send(s.to_string()).unwrap();
                    }
                    Err(e) => {
                        eprintln!("output was not utf8: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Error while reading: {}", e);
                break;
            }
        }
    }
    channel.wait_close().unwrap();
    println!("{}", channel.exit_status().unwrap());
}

pub fn new_connection_private_key(
    address: String,
    port: i32,
    username: Option<String>,
    key: PathBuf,
) -> anyhow::Result<Session> {
    let mut username_connect = username.as_ref().unwrap().as_str();
    if username.is_none() {
        username_connect = "root";
    }
    let tcp = TcpStream::connect(address + ":" + &port.to_string())?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    println!("Connecting: {}, {:?}", &username_connect, &key);
    let auth_result = sess.userauth_pubkey_file(&username_connect, None, &key, None)?;
    println!("{:?}", auth_result);
    Ok(sess)
}

pub fn new_connection_password(
    address: String,
    port: i32,
    username: Option<String>,
    password: String,
) -> anyhow::Result<Session> {
    let mut username_connect = username.as_ref().unwrap().as_str();
    if username.is_none() {
        username_connect = "root";
    }
    let tcp = TcpStream::connect(address + ":" + &port.to_string())?;
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    let auth_result = sess.userauth_password(&username_connect, &password)?;
    println!("{:?}", auth_result);
    Ok(sess)
}

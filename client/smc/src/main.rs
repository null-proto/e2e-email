use std::{fs::{read, read_to_string}, io::Write, net::{TcpListener, TcpStream}, os::linux::net::TcpStreamExt};

use protocol::{frame::field::{M_FROM, M_TITLE, M_TO}, kv::Kv, mail::File};

macro_rules! readln {
  ($i : expr) => {{
    print!($i);
    std::io::stdout().flush().unwrap();
    let mut s = String::new();
    _ = std::io::stdin().read_line(&mut s);
    s
  }};
}

fn main() {
  let env : Vec<String> = std::env::args().collect();
  if env.len() < 2 {
    let a = readln!("uu : ");
  }
  if env[1]=="read" {
    let listener = TcpListener::bind("[::1]:5555").unwrap();
    read_mail(listener);
  } else {
    send_mail(env[1].clone());
  }
}

fn send_mail(addr : String) {
  let title = readln!("title : ");
  let msg = readln!("msg : ");
  if let Ok(conn) = TcpStream::connect(&addr) {
    println!("connected : {}", addr);
    let mut conn = protocol::conn::Stream::new(conn);
    let from = "ee";
    let to = "you";

    let kv = Kv::build()
      .insert(M_FROM , from)
      .insert(M_TO , to)
      .insert(M_TITLE , &title)
      .finish();

    let file :File = (Kv::build().insert("ee", "ee").finish() , msg.as_bytes()).into();
    let mail = ( kv , Box::from(vec![file]) ).into();
    conn.send_mail(mail).unwrap();

    println!("MAIL SEND =====================");
  }
}

fn read_mail(listener : TcpListener) {
  while let Ok((conn, peer)) = listener.accept() {
    println!("connected : {}", peer);
    let mut conn = protocol::conn::Stream::new(conn);

    match conn.recv_mail() {
      Ok(mail) => {
      println!("NEW SEND =====================");
      println!("from  : {}", mail.from);
      println!("to    : {}", mail.to);
      println!("title : {}", mail.title);
      }

      Err(err) => println!("{:?}" , err)
    }
  }
}

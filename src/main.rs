use std::{
    env::{self},
    io::{BufWriter, Read, Write},
    net::TcpStream,
};

use eyre::{Context, OptionExt, Result};
use tracing::trace;

use crate::proto::{handshake, status, Serde};

mod proto;

fn main() -> Result<()> {
    setup();

    let mut args = env::args().skip(1);

    let server_addr = args.next().ok_or_eyre("must provide server address")?;
    let server_port: u16 = args
        .next()
        .map(|s| str::parse(&s))
        .transpose()?
        .unwrap_or(25565);

    let addr = format!("{server_addr}:{server_port}");

    trace!(addr, "connecting to server");
    let mut s = TcpStream::connect(&addr).wrap_err("failed to connect to server")?;

    let mut w = BufWriter::new(s.try_clone()?);

    trace!("sending handshake");
    send(
        &mut w,
        handshake::Req {
            proto_version: 767,
            server_addr: server_addr.to_string(),
            server_port,
            next_state: handshake::NextState::Status,
        },
    )?;

    trace!("sending status request");
    send(&mut w, status::Req {})?;

    trace!("waiting on status response");
    read(&mut s)?;

    Ok(())
}

fn send<M: Serde>(writer: &mut dyn Write, msg: M) -> Result<()> {
    let mut buf = vec![]; // XX: one allocation per write is absurd
    msg.encode(&mut buf)?;
    println!("{:x?}", buf);
    writer.write_all(&buf)?;
    writer.flush()?;
    Ok(())
}

// TODO
fn read(reader: &mut dyn Read) -> Result<Vec<u8>> {
    let mut buf = vec![0; 1024];
    loop {
        match reader.read(&mut buf)? {
            0 => break,
            n => {
                println!("read {n} bytes!");
                dbg!(&buf[..n]);
            }
        }
    }
    println!("done");
    Ok(vec![])
}

fn setup() {
    color_eyre::install().unwrap();
    tracing_subscriber::fmt::init();
}

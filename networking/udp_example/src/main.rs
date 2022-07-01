use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("localhost:34254")?;
    let mut buf = [0; 10];
    let (num, src) = socket.recv_from(&mut buf)?;
    let buf = &mut buf[..num];
    buf.reverse();
    socket.send_to(buf, src)?;
    Ok(())
}

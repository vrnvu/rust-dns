use std::net::UdpSocket;

mod dns;

fn main() -> anyhow::Result<()> {
    let domain_name = "google.com";
    let query = dns::DNSQuery::build_query(domain_name);

    // create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(("8.8.8.8", 53))?;
    socket.send(&query)?;

    // read the response
    let mut buf = [0; 1024];
    let (amt, _src) = socket.recv_from(&mut buf)?;

    println!("{:?}", &buf[..amt]);

    Ok(())
}

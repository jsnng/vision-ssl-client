extern crate tokio;

use tokio::net::UdpSocket;
use std::io;
use std::sync:Arc;

#[tokio::main]
async fn main() -> io::Result<()> {

    let args = Command::new("vision-ssl-recv")
    .version(crate_version!())
    .arg(
        Arg::new("local")
        .short("l")
        .long("local")
        .required(true)
        .value_name("IP:PORT")
        .help()
    )
    .arg(
        Arg::new("remote")
        .short("r")
        .long("remote")
        .required(true)
        .value_name("IP:PORT")
        .help()
    )
    .get_matches();

    let local_ip_addr: SocketAddr = args
    .get_one::<String>("local")
    .unwrap()
    .parse()
    .expect("invalid local addr");

    let remote_ip_addr: SocketAddr = args
    .get_one::<String>("remote")
    .unwrap()
    .parse()
    .expect("invalid remote addr");

    num_cpus = num_cpus::get()
    info!("{} cores available", num_cpus)

    let main_loop = tokio::spawn(async move {
        let mut buffer = [0; 1024];
        loop {
            let (len, addr) = udp
        }
    })
}


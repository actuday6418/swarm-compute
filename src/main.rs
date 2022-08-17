pub fn main() {
    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register(
        "_swarm_client._tcp.local".to_owned(),
        "libmdns Web Server".to_owned(),
        80,
        &["path=/"],
    );

    std::thread::spawn(f).join().unwrap().unwrap();
    println!("spawned");
    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}

use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Error, Record, RecordKind};
use std::{net::IpAddr, time::Duration};

const SERVICE_NAME: &'static str = "_swarm_client._tcp.local";

#[tokio::main]
async fn f() -> Result<(), Error> {
    // Iterate through responses from each Cast device, asking for new devices every 15s
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(15))?.listen();
    pin_mut!(stream);

    while let Some(Ok(response)) = stream.next().await {
        let addr = response.records().filter_map(self::to_ip_addr).next();

        if let Some(addr) = addr {
            println!("found cast device at {}", addr);
        } else {
            println!("cast device does not advertise address");
        }
    }

    Ok(())
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}

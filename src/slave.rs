use std::{
    error::Error,
    io::{Read, Write},
};

pub fn slave_loop() -> Result<(), Box<dyn Error>> {
    let listener = std::net::TcpListener::bind(("::", 4004))?;
    println!("Listening on localhost:4004");
    match listener.accept() {
        Ok((mut stream, _addr)) => {
            // read function from master
            let mut buffer = vec![0; 1024];
            stream.read(&mut buffer)?;
            let func: Box<dyn serde_traitobject::Fn(Vec<u8>) -> Vec<u8>> =
                match bincode::deserialize(&buffer) {
                    Ok(func) => func,
                    Err(e) => return Err(e),
                };

            // read data from master
            let mut buffer = vec![0; 1024];
            stream.read(&mut buffer)?;
            let a = func(buffer);
            stream.write_all(&a)?;
        }
        Err(addr) => return Err(format!("Connection to {addr:?} failed").into()),
    }
    Ok(())
}

pub fn spawn_slave() {
    loop {
        if let Err(e) = slave_loop() {
            println!("Recovered from failed transaction: {e}!");
        }
    }
}

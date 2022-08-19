use serde::de::DeserializeOwned;
use std::io::{Read, Write};
use std::net::TcpStream;

pub trait IteratorExt: Iterator
where
    Self::Item: serde::Serialize + DeserializeOwned,
{
    fn do_it(
        self,
        f: Box<dyn serde_traitobject::Fn(Vec<u8>) -> Vec<u8>>,
        manager: &mut TaskManager,
    );
}

impl<T> IteratorExt for T
where
    T: Iterator,
    <T as Iterator>::Item: serde::Serialize + DeserializeOwned + std::fmt::Debug,
{
    fn do_it(
        mut self,
        f: Box<dyn serde_traitobject::Fn(Vec<u8>) -> Vec<u8>>,
        manager: &mut TaskManager,
    ) where
        Self: Sized,
    {
        manager.job_track = Vec::new();
        while let Some(x) = self.next() {
            // loop until I find a free slave
            let slave: &mut (TcpStream, bool) = loop {
                if let Some(slave) = manager.slaves.iter_mut().find(|s| s.1 == false) {
                    break slave;
                }
            };
            // send code and data to slave
            slave
                .0
                .write(&bincode::serialize(&f).unwrap())
                .expect("Unable to speak to slave!");
            println!("Sent function");
            println!("{:?}", &bincode::serialize(&f).unwrap());
            slave
                .0
                .write(&bincode::serialize(&x).unwrap())
                .expect("Unable to speak to slave!");
            println!("Sent data");
            slave.1 = true;
            let mut buffer = vec![0; 1024];
            slave.0.read(&mut buffer).unwrap();
            let a = bincode::deserialize::<T::Item>(&buffer);
            println!("{:?}", a);

            // f.call_mut((&bincode::serialize(&x).unwrap(),));
        }
    }
}

pub struct TaskManager {
    /// Address of slave, and is he busy?
    slaves: Vec<(TcpStream, bool)>,
    /// Address of slave handling job, and is it done?
    job_track: Vec<(String, bool)>,
}

impl TaskManager {
    pub fn discover() -> Self {
        // find slaves
        let slaves = vec![(
            TcpStream::connect("127.0.0.1:4004").expect("Connection refused!"),
            false,
        )];
        TaskManager {
            slaves,
            job_track: Vec::new(),
        }
    }
}

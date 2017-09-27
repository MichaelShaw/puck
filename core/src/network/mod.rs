use std;
use std::thread;
use std::io;

use tokio_io::codec::length_delimited;
use tokio_io::{AsyncRead, AsyncWrite};

use futures::sync::oneshot;

pub mod client;
pub mod codec;
pub mod server;


#[derive(Debug)]
pub enum PuckNetworkError {
    IO(io::Error)
}

impl From<io::Error> for PuckNetworkError {
    fn from(err: io::Error) -> Self {
        PuckNetworkError::IO(err)
    }
}

pub type PuckNetworkResult<T> = Result<T, PuckNetworkError>;

pub fn bind_transport<T: AsyncRead + AsyncWrite>(io: T) -> length_delimited::Framed<T> {
    length_delimited::Framed::new(io) // by default a big endian u32 at the start
}

pub struct PoisonPill {
    pub sender : oneshot::Sender<u32>,
    pub join_handle : thread::JoinHandle<u32>,
}

impl PoisonPill {
    pub fn shutdown(self) -> std::result::Result<u32, std::boxed::Box<std::any::Any + std::marker::Send>> {
        self.sender.send(99).unwrap();
        self.join_handle.join()
    }
}
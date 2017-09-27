extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate bincode;

extern crate tokio_core;
extern crate tokio_io;
extern crate futures;
extern crate bytes;

pub mod event;
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

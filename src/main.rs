#![feature(slice_patterns)]

mod server;

fn main() {
    let port = 8000;
    server::serve(port);
}

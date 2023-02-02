fn main() {
    let server = Server::new("127.0.0.0:8080");
    server.run();
}

struct Server {
    addr: String,
}

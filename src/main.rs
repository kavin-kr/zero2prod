use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    zero2prod::run(address)?.await
}

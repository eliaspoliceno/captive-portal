use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::net::{IpAddr};

async fn generate204(req: HttpRequest) -> impl Responder {
    let start_str = &format!("Request \"{}\" URL=[{}] QueryString=[{}] Headers=[", req.method(), req.uri(), req.query_string());
    let mut owned_string: String = start_str.to_owned();

    let mut it = req.headers().iter().peekable();
    while let Some((key, value)) = it.next()  {
        if it.peek().is_none() {
            owned_string.push_str(&format!("{:?}: {:?}", key, value));
        } else {
            owned_string.push_str(&format!("{:?}: {:?}, ", key, value));
        }
    }
    
    let addr = req.peer_addr().unwrap().ip();
    if let IpAddr::V6(addr) = addr {
        if let Some(ipv4) = addr.to_ipv4_mapped() {
            owned_string.push_str(&format!("] Client=[{}] Host=[{}]", ipv4, req.connection_info().host()));
        } else {
            owned_string.push_str(&format!("] Client=[{}] Host=[{}]", addr, req.connection_info().host()));
        }
    }

    println!("{}", owned_string);
    format!("")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/generate_204", web::get().to(generate204))
    })
    .bind("[::]:8080")?
    .run()
    .await
}


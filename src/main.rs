use actix_web::{web, App, HttpRequest, HttpServer, Responder};

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

    owned_string.push_str("] ");
    if let Some(val) = req.peer_addr() {
    	let sub = val.ip().to_string();
    	if let Some(pos) = sub.rfind(':') {
    	    owned_string.push_str(&format!("Client=[{}] ", &sub[0..pos]));
    	} else {
    	    owned_string.push_str(&format!("Client=[{}] ", sub));
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


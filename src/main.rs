use actix_web::{web, App, HttpResponse, HttpServer, Responder};

struct AppState {
    http_client: reqwest::Client
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

async fn echo(body: String) -> impl Responder {
    HttpResponse::Ok().body(body)
}

async fn redirect(body: String, data: web::Data<AppState>) -> impl Responder{
    HttpResponse::Ok().body(data.http_client.get("https://www.baidu.com").send().await.unwrap().text().await.unwrap())
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                http_client: reqwest::Client::builder().
                    tcp_keepalive(std::time::Duration::from_secs(60))
                    .build().unwrap()
            })
            .route("/", web::get().to(index))
            .route("/echo", web::to(echo))
            .route("/redirect", web::to(redirect))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

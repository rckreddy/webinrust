
use actix_web::{get, HttpResponse, HttpServer, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_testpage))
                .bind(("127.0.0.1", 8080))?
                .run()
                .await
}

#[get("/")]
async fn get_testpage() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>Test Form</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Click Me</button>
                </form>
            "#,
        )
}
use actix_web::{
    web::{self},
    App, HttpResponse, HttpServer, Responder,
};
use ch2_2to2_3::gcd;
use serde::Deserialize;

/// # get_index
/// - path: "/"
async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="m"/>
        <button type="submit">Compute GCD</button>
        </form>
		"#,
    )
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring");
    }

    let mut response = format!(
        "The greatest common devisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd::gcd(form.n, form.m)
    );
    response.push_str(
        r#"
        <form action="/" method="get">
        <button type="submit">back</button>
        "#,
    );
    HttpResponse::Ok().content_type("text/html").body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            // service(get_index),
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    server.bind("127.0.0.1:9900")?.run().await
}

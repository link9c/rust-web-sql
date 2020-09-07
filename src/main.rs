mod sql;

use actix_files as fs;
use actix_web::{App, HttpServer, HttpResponse, web, HttpRequest, get};
use actix_files::NamedFile;
use sqlx::MySqlPool;
use anyhow::Result;
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io;
use std::fs::File;
use std::io::Read;


// async fn index2(obj: web::Path<MyObj>) -> Result<HttpResponse> {
//     Ok(HttpResponse::Ok().json(MyObj {
//         name: obj.name.to_string(),
//     }))
// }


#[get("/")]
async fn index(_req:HttpRequest) -> io::Result<NamedFile> {
    Ok(fs::NamedFile::open("static/index.html")?)
}

#[actix_rt::main]
async fn main() -> Result<()> {
    println!("start server at 0.0.0.0:8088");

    let mut file = File::open(".env")?;
    let mut database_url = String::new();
    file.read_to_string(&mut database_url)?;


    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    // let mut builder =
    //     SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    let  pool = MySqlPool::connect(&database_url).await?;
    HttpServer::new(move || {
        App::new()
            // .bind_openssl("127.0.0.1:8088", builder)?
            .data(pool.clone())
            // .route(r"/a/{name}", web::get().to(index2))
            // .route(r"/",web::get().to(index))
            .service(index)
            .configure(sql::init)
            .service(fs::Files::new("/static", "static").show_files_listing())
            .default_service(
                web::route().to(|| HttpResponse::NotFound()),
            )


    })
        .bind("0.0.0.0:8088")?
        .run()
        .await?;
    Ok(())
}
use routes::create_routes;

// 
mod routes;
pub async fn run() {
    let app = create_routes();
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}



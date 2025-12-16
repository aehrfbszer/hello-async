use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, decompression::RequestDecompressionLayer};

use crate::user::get_user;

mod user;
mod state;
mod app;
mod run;

#[tokio::main]
async fn main() {
    // // initialize tracing
    // tracing_subscriber::fmt::init();

    // // build our application with a route
    // let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root))
    //     .route("/users", get(get_user))
    //     // `POST /users` goes to `create_user`
    //     .route("/users", post(create_user))
    //     .layer(
    //         ServiceBuilder::new()
    //             .layer(RequestDecompressionLayer::new())
    //             .layer(CompressionLayer::new()),
    //     );

    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    run::run().unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    tracing::info!("root handler called");
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

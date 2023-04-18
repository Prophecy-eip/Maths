#![forbid(unsafe_code)]
#[warn(missing_copy_implementations)]
#[warn(missing_debug_implementations)]
#[warn(missing_docs)]
#[warn(trivial_casts)]
#[warn(trivial_numeric_casts)]
#[warn(clippy::all)]
pub mod dto;
pub mod fight;
pub mod global_test;
mod math_tools;
pub mod model;
pub mod modifier;
pub mod regiment;
pub mod stat;
pub mod web_server;

#[cfg(debug_assertions)]
async fn end() {
    std::process::exit(0)
}

async fn heartbeat() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

async fn make_prophecy(
    regiment: axum::Json<web_server::ProphecyRequest>,
) -> axum::response::Response {
    if !regiment
        .get_key()
        .eq(&std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string()))
    {
        return axum::response::IntoResponse::into_response((axum::http::StatusCode::UNAUTHORIZED, [(axum::http::header::CONTENT_TYPE, "text/plain")], "Missing or wrong key, if you should access this data please contact the administrators"));
    }
    let prophecies: maths::fight::FightPredictionResult = maths::fight::compute_turn(
        regiment.convert_attacking_position(),
        &regiment.convert_regiment(true),
        &regiment.convert_regiment(false),
    );
    let result: web_server::response::ProphecyResponse =
        web_server::response::ProphecyResponse::from_fight_prediction_result(prophecies);
    axum::response::IntoResponse::into_response(axum::Json(result))
}

#[tokio::main]
async fn main() {
    let app: axum::Router = axum::Router::new().route("/heartbeat", axum::routing::get(heartbeat));
    let app: axum::Router = app.route("/units", axum::routing::post(make_prophecy));
    #[cfg(debug_assertions)]
    let app: axum::Router = app.route("/end", axum::routing::get(end));

    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Error while creating the webserver");
}

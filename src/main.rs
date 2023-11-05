#![forbid(unsafe_code)]
#[warn(missing_copy_implementations)]
#[warn(missing_debug_implementations)]
#[warn(missing_docs)]
#[warn(trivial_casts)]
#[warn(trivial_numeric_casts)]
#[warn(clippy::all)]
pub mod ai_integration;
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

async fn make_prophecy_unit_vs_unit(
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
    let result: web_server::response::ProphecyResponseUnits =
        web_server::response::ProphecyResponseUnits::from_fight_prediction_result(prophecies);
    axum::response::IntoResponse::into_response(axum::Json(result))
}

async fn make_prophecy_army_vs_army(
    armies: axum::Json<web_server::ProphecyRequestArmies>,
) -> axum::response::Response {
    if !armies
        .get_key()
        .eq(&std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string()))
    {
        return axum::response::IntoResponse::into_response((axum::http::StatusCode::UNAUTHORIZED, [(axum::http::header::CONTENT_TYPE, "text/plain")], "Missing or wrong key, if you should access this data please contact the administrators"));
    }

    match ai_integration::implement_route::handle_flask_request(armies.0).await {
        Ok((score1, score2)) => {
            // Update the scores variable with the returned values
            let scores: web_server::response::ProphecyResponseArmies =
                web_server::response::ProphecyResponseArmies::new(score1, score2);
            axum::response::IntoResponse::into_response(axum::Json(scores))
        }
        Err(e) => {
            // Handle any other error from handle_flask_request as needed
            eprintln!("Error: {:?}", e);
            axum::response::IntoResponse::into_response((
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                [(axum::http::header::CONTENT_TYPE, "text/plain")],
                "Error in processing prediction",
            ))
        }
    }
}

#[tokio::main]
async fn main() {
    let app: axum::Router = axum::Router::new().route("/heartbeat", axum::routing::get(heartbeat));
    let app: axum::Router = app.route("/units", axum::routing::post(make_prophecy_unit_vs_unit));
    let app: axum::Router = app.route("/armies", axum::routing::post(make_prophecy_army_vs_army));
    #[cfg(debug_assertions)]
    let app: axum::Router = app.route("/end", axum::routing::get(end));

    let addr: std::net::SocketAddr = std::net::SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Error while creating the webserver");
}

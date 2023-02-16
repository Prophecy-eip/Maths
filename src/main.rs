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

#[actix_web::get("/heartbeat")]
async fn heartbeat() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
}

#[actix_web::post("/units")]
async fn make_prophecy(
    regiments: actix_web::web::Json<web_server::ProphecyRequest>,
) -> Result<actix_web::web::Json<web_server::response::ProphecyResponse>, actix_web::error::Error> {
    if !regiments
        .get_key()
        .eq(&std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string()))
    {
        return Err(actix_web::error::ErrorUnauthorized("Missing or wrong key, if you should access this data please contact the administrators"));
    }
    let prophecies: maths::fight::FightPredictionResult = maths::fight::compute_turn(
        regiments.convert_attacking_position(),
        &regiments.convert_regiment(true),
        &regiments.convert_regiment(false),
    );
    let result: web_server::response::ProphecyResponse =
        web_server::response::ProphecyResponse::from_fight_prediction_result(prophecies);
    Ok(actix_web::web::Json(result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    actix_web::HttpServer::new(|| {
        let logger: actix_web::middleware::Logger = actix_web::middleware::Logger::default();
        actix_web::App::new()
            .service(make_prophecy)
            .service(heartbeat)
            .wrap(logger)
            .app_data(
                actix_web::web::JsonConfig::default().error_handler(|err, _req| {
                    actix_web::error::InternalError::from_response(
                        "Missing element in request",
                        actix_web::HttpResponse::BadRequest()
                            .content_type("application/json")
                            .body(format!(r#"{{"statusCode": 400,"message":"{err}"}}"#)),
                    )
                    .into()
                }),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

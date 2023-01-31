pub mod fight;
pub mod global_test;
mod math_tools;
pub mod model;
pub mod modifier;
pub mod prediction;
pub mod regiment;
pub mod web_server;
pub mod stat;

#[actix_web::get("/heartbeat")]
async fn heartbeat() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
}

#[actix_web::post("/units")]
async fn make_prophecy(
    regiments: actix_web::web::Json<
        web_server::request_structures::make_prophecy::MakeProphecyRequest,
    >,
) -> actix_web::Result<impl actix_web::Responder> {
    if !regiments
        .get_key()
        .eq(&std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string()))
    {
        return Err(actix_web::error::ErrorUnauthorized("Missing or wrong key, if you should access this data please contact the administrators"));
    }
    let prophecies: std::collections::HashMap<
        maths::fight::ComputeCase,
        maths::prediction::Prediction,
    > = maths::fight::compute_turn(
        web_server::converter::web_objects::attacking_position_converter(
            regiments.get_attacking_position(),
        ),
        &web_server::converter::web_objects::regiment_converter(regiments.get_attacking_regiment()),
        &web_server::converter::web_objects::regiment_converter(regiments.get_defending_regiment()),
    );
    let result: web_server::response_structures::make_prophecy::MakeProphecyResponse =
        web_server::response_structures::make_prophecy::MakeProphecyResponse::from_dict(prophecies);
    Ok(actix_web::web::Json(result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(make_prophecy)
            .service(heartbeat)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

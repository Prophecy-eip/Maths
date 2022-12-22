#[derive(serde::Serialize, serde::Deserialize)]
struct Regiments {
    key: String,
    attacking_regiment: maths::regiment::Regiment,
    defending_regiment: maths::regiment::Regiment,
}

#[actix_web::get("/heartbeat")]
async fn heartbeat() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok()
}

#[actix_web::post("/units")]
async fn make_prophecy(
    regiments: actix_web::web::Json<Regiments>,
) -> actix_web::Result<impl actix_web::Responder> {
    if !regiments
        .key
        .eq(&std::env::var("PRIVATE_KEY").unwrap_or_else(|_| "".to_string()))
    {
        return Err(actix_web::error::ErrorUnauthorized("Missing or wrong key, if you should access this data please contact the administrators"));
    }
    let res: std::collections::HashMap<maths::fight::ComputeCase, maths::prediction::Prediction> =
        maths::fight::compute_turn(&regiments.attacking_regiment, &regiments.defending_regiment);
    Ok(actix_web::web::Json(res))
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

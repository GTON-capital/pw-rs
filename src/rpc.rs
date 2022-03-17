use actix_web::http::{header, header::*, StatusCode};
use actix_web::{get, web, HttpResponse, Responder};

use serde_derive::{Deserialize, Serialize};

use crate::client::Client;

fn apply_headers<B>(resp: &mut HttpResponse<B>) {
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    resp.headers_mut().insert(
        HeaderName::from_lowercase(b"access-control-allow-origin").unwrap(),
        HeaderValue::from_static("*"),
    );
    resp.headers_mut().insert(
        HeaderName::from_lowercase(b"access-control-allow-headers").unwrap(),
        HeaderValue::from_static("*"),
    );
    resp.headers_mut().insert(
        HeaderName::from_lowercase(b"access-control-allow-methods").unwrap(),
        HeaderValue::from_static("*"),
    );
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub result: T,
}

#[get("/rpc/base-price")]
pub async fn get_wftm_price(client: web::Data<Client>) -> impl Responder {
    let result = client.get_wftm_price().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

#[get("/rpc/owned/base-pool-lps")]
pub async fn get_wftm_gton_gc_pool_lp(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_wftm_gton_gc_pool_lp().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

#[get("/rpc/owned/usd-pool-lps")]
pub async fn get_usdc_gton_gc_pool_lp(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_usdc_gton_gc_pool_lp().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

#[get("/rpc/base-liquidity")]
pub async fn get_ftm_gton_liq(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_ftm_gton_liq().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

#[get("/rpc/usd-liquidity")]
pub async fn get_usdc_gton_liq(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_usdc_gton_liq().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

#[get("/rpc/base-pool-lps")]
pub async fn get_ftm_gton_lp(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_ftm_gton_lp().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

#[get("/rpc/gc-pol")]
pub async fn get_gc_pol(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_gc_pol().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

// @app.route('/rpc/pw-model-peg-with-pol-mln', methods=['GET'])
// def pwModelPegWithPolMln():
//   pol = request.args.get('pol', None) # with current liq is
//   gcFloor = request.args.get('gcFloor', None) # with current liq is
//   gcBias = request.args.get('gcBias', None) # with current liq is
//   gcMaxP = request.args.get('gcMaxP', None)
//   gcMaxL = request.args.get('gcMaxL', None)

//   return wrap_result(pwcalc.pwModelPegWithPolMln(
//     float(pol or 0),
//     float(gcFloor or 0), # with current liq is
//     float(gcBias or 0), # with current liq is
//     float(gcMaxP or 0),
//     float(gcMaxL or 1)
//   ))

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PWModelQueryArgs {
    pub pol: f64,
    #[serde(rename(serialize = "gcFloor"))]
    pub gc_floor: f64,
    #[serde(rename(serialize = "gcBias"))]
    pub gc_bias: f64,
    #[serde(rename(serialize = "gcMaxP"))]
    pub gc_max_p: f64,
    #[serde(rename = "gcMaxL")]
    pub gc_max_l: f64,
}

#[get("/rpc/pw-model-peg-with-pol-mln")]
pub async fn get_pw_model_with_pol_mln(
    payload: web::Query<PWModelQueryArgs>,
    client: web::Data<Client>,
) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x)
        .get_pw_model_with_pol_mln(
            payload.pol,
            payload.gc_floor,
            payload.gc_bias,
            payload.gc_max_p,
            payload.gc_max_l,
        )
        .await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

#[get("/rpc/gc-current-peg-usd")]
pub async fn get_gc_pw_current_peg_usd(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_gc_pw_current_peg_usd().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

// @app.route('/rpc/gc-current-peg-base', methods=['GET'])
// def getGCpwCurrentPegFTM():
//   return wrap_result(pwcalc.getGCpwCurrentPegFTM())
#[get("/rpc/gc-current-peg-base")]
pub async fn get_gc_pw_current_peg_ftm(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_gc_pw_current_peg_ftm().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

// @app.route('/rpc/base-to-usdc-price', methods=['GET'])
// def getGTONusdcPrice():
//   return wrap_result(pwcalc.getGTONusdcPrice())
#[get("/rpc/base-to-usdc-price")]
pub async fn get_gton_usdc_price(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_gton_usdc_price().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

// @app.route('/rpc/base-to-quote-price', methods=['GET'])
// def getGTONwftmPrice():
//   return wrap_result(pwcalc.getGTONwftmPrice())
#[get("/rpc/base-to-quote-price")]
pub async fn get_gton_wftm_price(client: web::Data<Client>) -> impl Responder {
    let client_l = Box::into_raw(Box::new(client));
    let x = unsafe { Box::from_raw(client_l) };
    let result = Box::leak(x).get_gton_wftm_price().await;

    let mut resp = HttpResponse::with_body(
        StatusCode::from_u16(200).unwrap(),
        serde_json::to_string(&Response { result }).unwrap(),
    );
    apply_headers(&mut resp);
    resp
}

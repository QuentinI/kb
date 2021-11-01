use std::sync::Arc;

use hyper::{body, Body, Request, Response, StatusCode};
use serde_json::json;

use crate::{
    error::{ApiError, ToApiError},
    Shared,
};

pub(crate) async fn insert(
    req: Request<Body>,
    shared: Arc<Shared>,
) -> Result<Response<Body>, ApiError> {
    let client = shared
        .pool
        .get()
        .await
        .into_api(StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = body::to_bytes(req.into_body())
        .await
        .map(|bytes| String::from_utf8(bytes.to_vec()))
        .into_api(StatusCode::BAD_REQUEST)?
        .into_api(StatusCode::BAD_REQUEST)?;

    client
        .execute(
            r#"
          INSERT INTO tickets
          SELECT id,
                 departure_code,
                 arrival_code,
                 to_timestamp(departure_time::integer) as arrival_time,
                 to_timestamp(arrival_time::integer) as arrival_time,
                 price
          FROM jsonb_populate_recordset(null::tickets_batch_pretransform, $1::text::jsonb->'tickets')
        "#,
            &[&body],
        )
        .await
        .into_api(StatusCode::BAD_REQUEST)?;

    Ok(Response::new(Body::from(json!({
        "status": "success"
    }).to_string())))
}

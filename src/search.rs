use std::sync::Arc;

use chrono::NaiveDate;
use hyper::{body, Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    error::{ApiError, ToApiError},
    Shared,
};

#[derive(Deserialize)]
struct Query {
    departure_code: String,
    arrival_code: String,
    departure_date: NaiveDate,
    limit: i64,
}

#[derive(Serialize)]
struct Solution {
    ticket_ids: Vec<String>,
    price: i32,
}

pub(crate) async fn search(
    req: Request<Body>,
    shared: Arc<Shared>,
) -> Result<Response<Body>, ApiError> {
    let mut client = shared
        .pool
        .get()
        .await
        .into_api(StatusCode::INTERNAL_SERVER_ERROR)?;

    let body = body::to_bytes(req.into_body())
        .await
        .map(|bytes| String::from_utf8(bytes.to_vec()))
        .into_api(StatusCode::BAD_REQUEST)?
        .into_api(StatusCode::BAD_REQUEST)?;

    let query: Query = serde_json::from_str(&body).into_api(StatusCode::BAD_REQUEST)?;

    let mut solutions = Vec::with_capacity((query.limit * 2) as usize);

    let tx = client
        .transaction()
        .await
        .into_api(StatusCode::INTERNAL_SERVER_ERROR)?;

    for row in tx
        .query(
            r#"
               SELECT id, price
               FROM tickets
               WHERE departure_code = $1
                AND arrival_code = $2
                AND departure_time > $3
                AND departure_time < $3 + '1 day'::interval
               ORDER BY price ASC
               LIMIT $4
            "#,
            &[
                &query.departure_code,
                &query.arrival_code,
                &query.departure_date.and_hms(0, 0, 0),
                &query.limit,
            ],
        )
        .await
        .into_api(StatusCode::BAD_REQUEST)?
    {
        solutions.push(Solution {
            ticket_ids: vec![row.get(0)],
            price: row.get(1),
        });
    }

    for row in tx
        .query(
            r#"
               SELECT depart.id, arrive.id, depart.price + arrive.price
               FROM tickets depart
               JOIN tickets arrive ON arrive.departure_code = depart.arrival_code
               WHERE  depart.departure_code = $1
                 AND  arrive.arrival_code   = $2
                 AND  depart.departure_time > $3
                 AND  depart.departure_time < $3 + '1 day'::interval
                 AND  arrive.departure_time - depart.arrival_time > '3 hours'::interval
                 AND  arrive.departure_time - depart.arrival_time < '12 hours'::interval
               ORDER BY depart.price + arrive.price ASC
               LIMIT $4
            "#,
            &[
                &query.departure_code,
                &query.arrival_code,
                &query.departure_date.and_hms(0, 0, 0),
                &query.limit,
            ],
        )
        .await
        .into_api(StatusCode::BAD_REQUEST)?
    {
        solutions.push(Solution {
            ticket_ids: vec![row.get(0), row.get(1)],
            price: row.get(2),
        });
    }

    solutions.sort_by_key(|s| s.price);

    Ok(Response::new(Body::from(
        json!({
            "solutions": solutions.chunks(query.limit as usize).next().unwrap_or_default()
        })
        .to_string(),
    )))
}

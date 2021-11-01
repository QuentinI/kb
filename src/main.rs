use std::env;
use std::{convert::Infallible, net::SocketAddr, sync::Arc};

use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{Config, Pool};
use dotenv::dotenv;
use hyper::{header::HeaderValue, service, Body, Method, Request, Response, Server, StatusCode};
use serde_json::json;

pub(crate) mod error;
pub(crate) mod insert;
pub(crate) mod search;

use insert::insert;
use search::search;

struct Shared {
    pub pool: Pool,
}

async fn handle(req: Request<Body>, shared: Arc<Shared>) -> Result<Response<Body>, Infallible> {
    let res = match (req.method(), req.uri().path()) {
        (&Method::POST, "/batch_insert") => insert(req, shared).await,
        (&Method::POST, "/search") => search(req, shared).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()),
    };

    let mut res = res.unwrap_or_else(|e| {
        Response::builder()
            .status(e.status)
            .body(Body::from(
                json!({
                    "error": e.error
                })
                .to_string(),
            ))
            .unwrap()
    });

    (*res.headers_mut()).insert(
        "Content-Type",
        HeaderValue::from_str("application/json").unwrap(),
    );

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv()?;

    let mut cfg = Config::new();
    cfg.dbname = Some("postgres".to_string());
    cfg.user = env::var("POSTGRES_USER".to_string()).ok();
    cfg.password = env::var("POSTGRES_PASSWORD".to_string()).ok();
    cfg.host = Some(env::var("POSTGRES_HOST".to_string()).unwrap_or("localhost".into()));
    cfg.port = env::var("POSTGRES_PORT".to_string())
        .unwrap_or("5432".into())
        .parse()
        .ok();
    let pool = cfg.create_pool(None, NoTls)?;
    let shared = Arc::new(Shared { pool });

    let make_service = service::make_service_fn(|_conn| {
        let shared_arc = shared.clone();
        async {
            Ok::<_, Infallible>(service::service_fn(move |req| {
                handle(req, shared_arc.clone())
            }))
        }
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let server = Server::bind(&addr).serve(make_service);
    server.await.unwrap();

    Ok(())
}

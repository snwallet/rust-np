#![deny(warnings)]

mod controller;

mod model;

use hyper::{Body, Method, Request, Response,};

use crate::app::controller::post_controller::*;


use mysql::{PooledConn, Pool};

pub fn db_conn() -> PooledConn {
    let dsn = String::from("mysql://root:root@192.168.0.123:3306/test");
    let pool = Pool::new(dsn).unwrap();
    pool.get_conn().unwrap()
}


pub async fn router(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {

    match (req.method(), req.uri().path()) {

        (&Method::POST, "/test") => test_post(req).await,
        (&Method::POST, "/get_token") => get_token(req).await,
        (&Method::POST, "/all_token") => all_token(req).await,
        (&Method::POST, "/tokens") => tokens(req).await,
        (&Method::POST, "/tokendesc") => tokendesc(req).await,
        (&Method::POST, "/apps") => apps(req).await,
        (&Method::POST, "/register") => register(req).await,
        _ => nofound(),
    }
}
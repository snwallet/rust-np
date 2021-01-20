#![deny(warnings)]

// use futures_util::TryStreamExt;
use hyper::{Body, Request, Response, StatusCode, header, HeaderMap};

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use url::form_urlencoded;
use crate::app::model::help_model;
use base64::{decode};

#[derive(Serialize, Deserialize)]
struct JsonResult<T> { errmsg:String, data:T, errno:i8 }




fn string_response(str:String)->Result<Response<Body>, hyper::Error>{
    Ok(Response::builder()
           .status(StatusCode::OK)
           .header(header::CONTENT_TYPE, "application/json")
           .body(Body::from(str)).unwrap())
}


 fn nodata_response(number:i8,msg:String)->Result<Response<Body>, hyper::Error>{
    let res =  JsonResult{
        errno:number,
        errmsg: msg,
        data: ""
    };
    let json = serde_json::to_string(&res).unwrap();
    Ok(Response::builder()
           .status(StatusCode::OK)
           .header(header::CONTENT_TYPE, "application/json")
           .body(Body::from(json)).unwrap())
}

fn check_response(mp:&HeaderMap,str:String)->Result<Response<Body>, hyper::Error>{
    if let Some(token) = mp.get("token") {
        if help_model::check_token(&decode(token).unwrap()[..]) {
            string_response(str)
        }else{
            nodata_response(-1,"token error".to_string())
        }
    }else{
        nodata_response(-1,"param error".to_string())
    }
}

// 404
pub fn nofound() -> Result<Response<Body>, hyper::Error>{
    nodata_response(-1,"404".to_string())
}

//post param as json
pub async fn test_post(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let hd = req.headers();
    // println!("{:?}",hd);
    let res = JsonResult{
        errno: 0,
        errmsg: "success".to_string(),
        data:crate::app::model::help_model::get_token()
    };
    let json = serde_json::to_string(&res).unwrap();
    check_response(hd,json)
}

//get a token
pub async fn get_token(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let b = hyper::body::to_bytes(req).await?;
    let params = form_urlencoded::parse(b.as_ref())
        .into_owned()
        .collect::<HashMap<String, String>>();
    println!("{:?}",params);
    if let (Some(name),Some(pwd)) = (params.get("name"),params.get("pwd") ) {
        if (name,pwd)==(&"test".to_string(),&"123456".to_string()) {
            let res = JsonResult{
                errno: 0,
                errmsg: "success".to_string(),
                data:crate::app::model::help_model::get_token()
            };
            let json = serde_json::to_string(&res).unwrap();
            string_response(json)
        }else{
            nodata_response(-1,"param error".to_string())
        }
    } else {
         nodata_response(-1,"param error".to_string())
    }

}

pub async fn all_token(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let hd = req.headers();
    // println!("{:?}",hd);
    let res = JsonResult{
        errno: 0,
        errmsg: "success".to_string(),
        data:crate::app::model::help_model::get_token()
    };
    let json = serde_json::to_string(&res).unwrap();
    check_response(hd,json)
}

pub async fn tokens(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let hd = req.headers();
    // println!("{:?}",hd);
    let res = JsonResult{
        errno: 0,
        errmsg: "success".to_string(),
        data:crate::app::model::help_model::get_token()
    };
    let json = serde_json::to_string(&res).unwrap();
    check_response(hd,json)
}

pub async fn tokendesc(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let hd = req.headers();
    // println!("{:?}",hd);
    let res = JsonResult{
        errno: 0,
        errmsg: "success".to_string(),
        data:crate::app::model::help_model::get_token()
    };
    let json = serde_json::to_string(&res).unwrap();
    check_response(hd,json)
}

pub async fn apps(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let hd = req.headers();

    let res = JsonResult{
        errno: 0,
        errmsg: "success".to_string(),
        data:crate::app::model::apps_model::select()
    };
    let json = serde_json::to_string(&res).unwrap();
    check_response(hd,json)
}

pub async fn register(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let hd = Request::headers(&req);
    // let b = hyper::body::to_bytes(&req).await?;
    // println!("{:?}",b);
    // let params = form_urlencoded::parse(b.as_ref())
    //     .into_owned()
    //     .collect::<HashMap<String, String>>();
    // println!("{:?}",params);
        let ip = "192.168.0.1".to_string();
        let res = JsonResult{
            errno: 0,
            errmsg: "success".to_string(),
            data:crate::app::model::user_model::_insert("0x21312312".to_string(),ip)
        };
        let json = serde_json::to_string(&res).unwrap();
        check_response(hd,json)



}
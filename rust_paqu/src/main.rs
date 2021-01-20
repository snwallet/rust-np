//use futures::future::join_all;
//
//
//async fn fetch_path(path:String) -> surf::Result<String>{
//    let mut back_string = String::new();
//    match surf::get(&path).await {
//        Ok(mut response) => {
//            match response.body_string().await{
//                Ok(text) =>{
//                    println!("Read response text {},{}" ,text.len(),text);
//                    back_string = format!("Read response text {} \t {}\t {}",path,text.len(),text)
//                }
//                Err(_) => {
//                    println!("Read response text Error!")
//                }
//            };
//        }
//        Err(_) => {
//            println!("reqwest get Error!")
//        }
//    }
//    Ok(back_string)
//}
//
//
//#[async_std::main]
//async fn main() -> surf::Result<()>{
//    let paths = vec![
//        "https://www.cnblogs.com/coderw/p/14294511.html".to_string(),
//        "https://www.cnblogs.com/".to_string()
//    ];
//    let result_list = join_all(paths.into_iter().map(|path|{
//        fetch_path(path)
//    })).await;
//
//    let mut list_string:Vec<String> = vec![];
//    for ele in result_list.into_iter(){
//        if ele.is_ok(){
//            list_string.push(ele.unwrap())
//        }else {
//            return Err(ele.unwrap_err())
//        }
//    }
//
//    println!("请求输出：{:?}",list_string);
//    Ok(())
//}
use std::fs::File;
//use std::io::Write;
use std::io::Cursor;

fn main(){
    let file = File::open("02.mp3").unwrap();
    let (_stream,handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let file_source = rodio::Decoder::new(file).unwrap();
    sink.append(file_source);
    sink.sleep_until_end();

//    let resp = reqwest::blocking::get(
//        "https://go-sycdn.kuwo.cn/1f96d895370b540744075caeee1c32ba/600797c1/resource/n1/66/25/442969063.mp3").unwrap();
////
////    let mut file = File::create("02.mp3").unwrap();
////    //file.write_all(&resp.bytes().expect("Unable to read the data")).expect("Unable to write the data");
////
////    std::io::copy(&mut resp,&mut file).expect("Unable to copy the data");
//    let resp = reqwest::blocking::get(
//        "https://go-sycdn.kuwo.cn/1f96d895370b540744075caeee1c32ba/600797c1/resource/n1/66/25/442969063.mp3").unwrap();
//    let (_stream,handle) = rodio::OutputStream::try_default().unwrap();
//    let sink = rodio::Sink::try_new(&handle).unwrap();
//
//    let cursor = Cursor::new(resp.bytes().unwrap());
//    let url_source = rodio::Decoder::new(cursor).unwrap();
//    sink.append(url_source);
//    sink.sleep_until_end();
}

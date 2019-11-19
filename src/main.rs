#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use std::io::{Write};
use std::path::Path;
use std::vec;
use curl::easy::Easy;
use std::ffi::OsStr;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::JsonValue;
use std::fs::{File};
use base64::encode;
use image::FilterType;

fn load_and_save_img(img_url :&str) -> String {
    let mut easy = Easy::new();
    let img_ext = Path::new(img_url).extension().and_then(OsStr::to_str).unwrap();
    let file_name = format!("{}.{}", encode(img_url), img_ext);
    let mut file = File::create(format!("static/{}", file_name)).unwrap();
    easy.url(img_url).unwrap();
    easy.write_function(move |data| {
        file.write_all(data);
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    return file_name;
}

fn resize_image(img_name : String) {
    image::open(format!("static/{}", img_name))
            .unwrap()
            .resize(100, 100, FilterType::Nearest)
            .save(format!("static/{}", img_name))
            .unwrap();
}

#[post("/ResizeImage", data="<urls>")]
fn get_link_image(urls : String) -> JsonValue {
    let vec_urls = urls.split(" ").collect::<Vec<&str>>();
    let mut res = Vec::new(); 
    for url in vec_urls {
        let img_name = load_and_save_img(url);
        res.push(format!("/img100/{}", img_name.to_owned()));
        resize_image(img_name);
    }
    json!({"result" : res})
}

fn main() {
    rocket::ignite()
        .mount("/", routes![get_link_image])
        .mount("/img100/", StaticFiles::from("static"))
        .launch();
}
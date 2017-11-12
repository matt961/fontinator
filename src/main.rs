#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate tokio_core;
extern crate hyper;
extern crate html5ever;
extern crate rocket;
extern crate futures;
extern crate hyper_tls;

mod fontrand;
mod downloader;

use tokio_core::reactor::Core;

use hyper::Uri;
use hyper::error::Error;

use rocket::http::RawStr;
use rocket::request::{FromParam, FromForm, FromFormValue, Form};
use rocket::response::content;

use std::str::FromStr;

#[post("/url", data="<url>")]
fn font_randomizer(url: Form<Url>) -> content::Html<String> {
    if let Ok(uri) = Uri::from_str(url.get().inner.as_str()) {
        if let Ok(result) = downloader::download_page(uri) {
            if let Ok(html) = String::from_utf8(result) {
                content::Html(html)
            } else {
                content::Html("Couldn't parse a String".to_string())
            }
        } else {
            content::Html("Couldn't download page :(".to_string())
        }
    } else {
        content::Html("Very unexpected behavior!".to_string())
    }
}

#[get("/")]
fn hello() -> content::Html<&'static str> {
    content::Html(
        include_str!("index.html")
    )
}

fn main() {
    rocket::ignite()
        .mount("/", routes![font_randomizer, hello]).launch();
}

#[derive(Debug, FromForm)]
struct Url {
    #[form(field = "url")]
    inner: String
}

impl<'a> FromFormValue<'a> for Url {
    type Error = Error;

    fn from_form_value(value: &'a RawStr) -> Result<Self, Self::Error> {
        match Uri::from_str(value) {
            Ok(uri) => {
                let val = Url {
                    inner: uri.to_string()
                };
                Ok(val)
            },
            Err(e) => Err(Error::Uri(e))
        }
    }
}

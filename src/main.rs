#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate tokio_core;
extern crate hyper;
#[macro_use(ns, local_name, namespace_url)]
extern crate html5ever;
extern crate markup5ever;
extern crate rocket;
extern crate futures;
extern crate hyper_tls;
extern crate rand;

mod manipulation;
mod downloader;

use tokio_core::reactor::Core;

use hyper::Uri;

use rocket::http::RawStr;
use rocket::request::{
    //    FromParam,
    //    FromForm,
    FromFormValue,
    Form,
    Request
};
use rocket::response::{
    content,
    NamedFile
};

use std::str::FromStr;

use std::io;

use std::path::{
    Path,
    PathBuf
};

use futures::{Future, Stream};

#[post("/url", data = "<url>")]
fn font_randomizer(url: Form<Url>) -> Result<content::Html<Vec<u8>>, String> {
    let mut core = Core::new().map_err(|_| {
        "Could not create an event loop :(".to_string()
    })?;

    let url: Uri = url.get().inner.parse().map_err(|_| {
        "Not a real URL :(".to_string()
    })?;

    let scheme = url.scheme().ok_or("https")?; // default to https if there is no scheme
    let domain = url.authority().ok_or("")?; // defaults to no domain string

    let handle = &core.handle();
    let client = downloader::new_http_client(handle)?;
    let download = downloader::download_page(url.clone(), client)
        .and_then(|body| {
            body.concat2()
        })
        .map_err(|_| {
            "Had trouble downloading web page :(".to_string()
        })
        .map(|all| all.to_vec());
    let html_bytes = core.run(download)?;
    let dom = manipulation::to_dom(&html_bytes);
    let mut bytes = Vec::new();
    manipulation::push_style(dom.document.clone())?;
    manipulation::walk_and_randomize(dom.document.clone(), scheme, domain);
    html5ever::serialize(&mut bytes, &dom.document, Default::default())
        .map_err(|_| "Couldn't serialize your page, sorry :(".to_string())?;
    Ok(content::Html(bytes.to_vec()))
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/css")]
fn style() -> io::Result<NamedFile> {
    NamedFile::open("static/style.css")
}

#[error(404)]
fn not_found(req: &Request) -> String {
    "Hello there :~)".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![font_randomizer, index, style])
        .catch(errors![not_found])
        .launch();
}

#[derive(Debug, FromForm)]
struct Url {
    #[form(field = "url")]
    inner: String
}

impl<'a> FromFormValue<'a> for Url {
    type Error = hyper::Error;

    fn from_form_value(value: &'a RawStr) -> Result<Self, Self::Error> {
        match Uri::from_str(value) {
            Ok(uri) => {
                let val = Url {
                    inner: uri.to_string()
                };
                Ok(val)
            }
            Err(e) => Err(hyper::Error::Uri(e))
        }
    }
}

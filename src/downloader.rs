use tokio_core::reactor::{Core, Handle};

use hyper_tls::HttpsConnector;

use hyper::client::{Client, HttpConnector};
use hyper::Uri;
use hyper::Body;
use hyper::StatusCode;

use futures::{Future, Stream};

use std::io::{self, Write};

pub fn download_page(uri: Uri) -> io::Result<Vec<u8>> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let client = new_http_client(&handle)?;

    let work = client.get(uri).and_then(|res| {
        println!("{}\n", res.headers());
        println!("{}\n", res.status());
        res.body().concat2()
    }).map(|body| {
        io::stdout().write_all(&body);
        body
    });
    let page = core.run(work).unwrap();
    Ok(page.to_vec())
}

fn new_http_client(handle: &Handle)
                   -> Result<Client<HttpsConnector<HttpConnector>>, io::Error> {
    let new_https = HttpsConnector::new(1, &handle);
    if let Ok(https_cnctr) = new_https {
        Ok(Client::configure()
            .connector(https_cnctr)
            .build(&handle))
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Could not establish an HttpsConnector."))
    }
}
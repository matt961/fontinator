use tokio_core::reactor::Handle;

use hyper_tls::HttpsConnector;

use hyper::client::{Client, HttpConnector};

use hyper::{
    Uri,
    Body,
    //    StatusCode,
    Error
};

use futures::{self, Future};

type HttpsClient = Client<HttpsConnector<HttpConnector>>;

type DownloadPage = Box<Future<Item=Body, Error=Error>>;

pub fn download_page(uri: Uri, client: HttpsClient) -> DownloadPage {
    Box::new(
        client.get(uri)
            .and_then(|res| {
                futures::future::ok(res.body())
            }))
}

pub fn new_http_client(handle: &Handle)
                       -> Result<HttpsClient, String> {
    HttpsConnector::new(1, handle)
        .map(|https| {
            Client::configure()
                .connector(https)
                .build(handle)
        })
        .map_err(|_| "Couldn't create a connection :(".to_string())
}
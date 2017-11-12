# fontinator
Trying out Rocket + Tokio stuff by creating a page re-server that randomizes fonts

# Trying it out
Clone this repo and execute the following command:
```
cargo run
```
this runs the server on `localhost:8000` .

Requesting the root path loads a form which takes a URL. The URL will be downloaded (and modified!) and sent as a 
response body. Note that the page will look messed up 99% of the time because ORIGIN is different from the 
original page's in this server's response. I _could_ fix this, but it seems like too much hassle for a school project.

use std::io::prelude::*;
use std::io;
use std::io::{Error, ErrorKind};
use hyper::Client;
use hyper::client::{RequestBuilder, Body};
use hyper::header::{Headers, Authorization};

pub struct DBClient {
    hypcli: Client,
    token: String,
}

impl DBClient {
    pub fn new() -> DBClient {
        DBClient {
            hypcli: Client::new(),
            token: String::new(),
        }
    }

    pub fn new_with_token(token: String) -> DBClient {
        DBClient {
            hypcli: Client::new(),
            token: token,
        }
    }

    pub fn get_file(&self, path: String) -> DBFile {
        let mut headers = Headers::new();
        headers.set(Authorization(self.token.clone()));
        DBFile::new(&self.hypcli, path, headers)
    }
}


static FILE_PUT: &'static str = "https://content.dropboxapi.com/1/files_put/auto/";

pub struct DBFile<'c> {
    client: &'c Client,
    path: String,
    oauth: Headers,
}

impl<'c> DBFile<'c> {
    fn new(client: &Client, path: String, oauth: Headers) -> DBFile {
        DBFile {
            client: client,
            path: path,
            oauth: oauth,
        }
    }
}

impl<'c> Write for DBFile<'c> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let uri = FILE_PUT.to_string() + &self.path;
        let mut putreq = self.client.put(&uri);
        let size = buf.len();
        let body = Body::BufBody(buf, size);

        putreq = putreq.body(body);
        putreq = putreq.headers(self.oauth.clone());

        let response = match putreq.send() {
            Ok(o) => o,
            Err(e) => panic!(e),
        };
        Err(Error::new(ErrorKind::Other, format!("{}", response.status)))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

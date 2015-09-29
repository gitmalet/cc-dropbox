use std::io::prelude::*;
use std::io;
use std::io::{Error, ErrorKind};
use hyper::Client;
use hyper::client::{RequestBuilder, Body};
use hyper::header::{Headers, Authorization};
use hyper::status::StatusCode;

use metadata::FileMetaData;

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

static FILE: &'static str = "https://content.dropboxapi.com/1/files/auto/";
static FILE_PUT: &'static str = "https://content.dropboxapi.com/1/files_put/auto/";

pub struct DBFile<'c> {
    client: &'c Client,
    path: String,
    oauth: Headers,
    pub lastmsg: Option<FileMetaData>,
}

impl<'c> DBFile<'c> {
    fn new(client: &Client, path: String, oauth: Headers) -> DBFile {
        DBFile {
            client: client,
            path: path,
            oauth: oauth,
            lastmsg: None,
        }
    }
}

impl<'c> Write for DBFile<'c> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let uri = FILE_PUT.to_string() + &self.path;
        let mut req = self.client.put(&uri);
        let size = buf.len();
        let body = Body::BufBody(buf, size);

        req = req.body(body);
        req = req.headers(self.oauth.clone());

        let mut response = match req.send() {
            Ok(o) => o,
            Err(e) => panic!(e),
        };
        match response.status {
            StatusCode::Ok => {},
            e @ _ => return Err(Error::new(ErrorKind::Other, format!("{}", e)))
        };
        let mut body = String::new();
        response.read_to_string(&mut body);
        //let mut lines = body.lines().map(|s| s.to_string().split(": "));
        Ok(size)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<'c> Read for DBFile<'c> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let uri = FILE.to_string() + &self.path;
        let mut req = self.client.get(&uri);

        req = req.headers(self.oauth.clone());

        let response = match req.send() {
            Ok(o) => o,
            Err(e) => panic!(e),
        };
        match response.status {
            StatusCode::Ok => Ok(0usize),
            e @ _ => Err(Error::new(ErrorKind::Other, format!("{}", e)))
        }
    }
}

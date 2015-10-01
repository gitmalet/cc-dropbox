use std::io::prelude::*;
use std::io;
use std::io::{Error, ErrorKind};
use hyper::Client;
use hyper::client::{RequestBuilder, Body};
use hyper::header::{Headers, Authorization, ContentType};
use hyper::status::StatusCode;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use serde_json;

use metadata::MetaData;

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
        DBFile::new(&self.hypcli, path, self.token.clone())
    }
}

static FILE: &'static str = "https://content.dropboxapi.com/1/files/auto/";
static FILE_PUT: &'static str = "https://content.dropboxapi.com/1/files_put/auto/";

//static FILE: &'static str = "http://192.168.0.1/";
//static FILE_PUT: &'static str = "http://192.168.0.1/";

pub struct DBFile<'c> {
    client: &'c Client,
    path: String,
    token: String,
    pub lastmsg: Option<MetaData>,
}

impl<'c> DBFile<'c> {
    fn new(client: &Client, path: String, token: String) -> DBFile {
        DBFile {
            client: client,
            path: path,
            token: token,
            lastmsg: None,
        }
    }
}

impl<'c> Write for DBFile<'c> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let uri = FILE_PUT.to_string() + &self.path;
        let mut req = self.client.put(&uri);
        let size = buf.len();

        let mut headers = Headers::new();
        headers.set(Authorization(self.token.clone()));
        headers.set(
            ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![(Attr::Charset, Value::Utf8)]))
        );

        let body = Body::BufBody(buf, size);

        req = req.headers(headers);
        req = req.body(body);

        let mut response = match req.send() {
            Ok(o) => o,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("{}", e))),
        };

        match response.status {
            StatusCode::Ok => {},
            e @ _ => return Err(Error::new(ErrorKind::Other, format!("{}", e)))
        };
        let mut body = String::new();
        response.read_to_string(&mut body);
        self.lastmsg = match serde_json::from_str(&body) {
            Ok(o) => o,
            Err(e) => return Err(Error::new(ErrorKind::Other,
                format!("Error on parsing response: {}, String to parse was:\n{}", e, body
            ))),
        };
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

        let mut headers = Headers::new();
        headers.set(Authorization(self.token.clone()));

        req = req.headers(headers);

        let response = match req.send() {
            Ok(o) => o,
            Err(e) => return Err(Error::new(ErrorKind::Other, format!("{}", e))),
        };
        match response.status {
            StatusCode::Ok => Ok(0usize),
            e @ _ => Err(Error::new(ErrorKind::Other, format!("{}", e)))
        }
    }
}

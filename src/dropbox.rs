use std::io::prelude::*;
use std::io;
use hyper::Client;
use hyper::error;
use hyper::client::{RequestBuilder, Body};
use hyper::header::{Headers, Authorization, ContentType};
use hyper::status::StatusCode;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use url::form_urlencoded;
use serde_json;

use metadata::MetaData;
use metadata::TokenResponse;

pub struct DBClient {
    hypcli: Client,
    token: String,
    client_key: String,
    client_secret: String,
}

static AUTHORIZE: &'static str = "https://www.dropbox.com/1/oauth2/authorize?response_type=code&client_id=";
static TOKEN: &'static str = "https://api.dropboxapi.com/1/oauth2/token";

impl DBClient {
    pub fn new(client_key: String, client_secret: String) -> DBClient {
        DBClient {
            hypcli: Client::new(),
            client_key: client_key,
            client_secret: client_secret,
            token: String::new(),
        }

    }

    pub fn new_with_token(client_key: String, client_secret: String, token: String) -> DBClient {
        DBClient {
            hypcli: Client::new(),
            client_key: client_key,
            client_secret: client_secret,
            token: token,
        }
    }

    pub fn get_file(&self, path: String) -> DBFile {
        DBFile::new(&self.hypcli, path, self.token.clone())
    }

    pub fn get_authorize_url(&self) -> String {
        format!("{}{}", AUTHORIZE, self.client_key)
    }

    pub fn set_token(&mut self, code: &str) -> error::Result<String>{
        //TODO: Better Error handling
        let body = form_urlencoded::serialize(
            vec![("code", code), ("grant_type", "authorization_code"),
            ("client_id", &self.client_key), ("client_secret", &self.client_secret)].into_iter());

        let mut response = try!(self.hypcli.post(TOKEN)
            .body(&body).send());

        //TODO: extract Token from response
        let mut body = String::new();
        try!(response.read_to_string(&mut body));

        match response.status {
            StatusCode::Ok => {},
            e @ _ => return Err(error::Error::from(io::Error::new(io::ErrorKind::Other, format!("{}", e)))),
        };

        let tokenresponse: TokenResponse = match serde_json::from_str(&body) {
            Ok(o) => o,
            Err(e) => return Err(error::Error::from(io::Error::new(io::ErrorKind::Other, format!("{}", e)))),
        };

        self.token = String::from("Bearer ") + &tokenresponse.access_token;
        Ok(self.token.clone())
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
        let size = buf.len();

        let mut headers = Headers::new();
        headers.set(Authorization(self.token.clone()));
        headers.set(
            ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![(Attr::Charset, Value::Utf8)]))
        );

        let mut response = match self.client.put(&uri)
            .headers(headers).body(buf).send() {
                Ok(o) => o,
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("{}", e))),
            };

        match response.status {
            StatusCode::Ok => {},
            e @ _ => return Err(io::Error::new(io::ErrorKind::Other, format!("{}", e)))
        };

        let mut body = String::new();
        try!(response.read_to_string(&mut body));

        self.lastmsg = match serde_json::from_str(&body) {
            Ok(o) => o,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other,
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

        let mut headers = Headers::new();
        headers.set(Authorization(self.token.clone()));

        let mut response = match self.client.get(&uri)
            .headers(headers).send() {
                Ok(o) => o,
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, format!("{}", e))),
            };

        match response.status {
            StatusCode::Ok => Ok(0usize),
            e @ _ => Err(io::Error::new(io::ErrorKind::Other, format!("{}", e)))
        }
    }
}

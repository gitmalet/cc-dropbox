#![feature(box_syntax, custom_derive, plugin, append, split_off)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate url;
extern crate chrono;
extern crate serde;
extern crate serde_json;

pub mod dropbox;
pub mod metadata;

#[cfg(test)]
mod tests {

    use std::io::{Write, Read};
    use dropbox::DBClient;
    use metadata::MetaData;

    #[test]
    fn test_new() {
        DBClient::new(String::new(), String::new());
    }

    #[test]
    fn test_sample_file_put() {
        let f = b"Hello world!";
        let token = include_str!("oauth_token").to_string();
        let dbc = DBClient::new_with_token(String::new(), String::new(), token);
        let mut dbf = dbc.get_file("test1.txt".to_string());
        dbf.write_all(f).unwrap();
    }

    #[test]
    fn test_sample_file_get() {
        let f = b"Hello world!";
        let token = include_str!("oauth_token").to_string();
        let dbc = DBClient::new_with_token(String::new(), String::new(), token);
        let mut dbf = dbc.get_file("test1.txt".to_string());
        let mut rf: Vec<u8> = Vec::with_capacity(f.len());
        dbf.read_to_end(&mut rf).unwrap();
        for (a, b) in f.iter().zip(rf) {
            assert_eq!(*a, b);
        }
    }

    #[test]
    fn test_sample_metadata() {
        let f = b"Hello world!";
        let token = include_str!("oauth_token").to_string();
        let dbc = DBClient::new_with_token(String::new(), String::new(), token);
        let mut dbf = dbc.get_file("test1.txt".to_string());
        dbf.write_all(f).unwrap();

        let meta: MetaData = match dbf.lastmsg {
            Some(s) => s,
            None => panic!("No metadata found"),
        };
        assert_eq!(&meta.thumb_exists, &false);
        assert_eq!(&meta.bytes, &f.len());
    }
}



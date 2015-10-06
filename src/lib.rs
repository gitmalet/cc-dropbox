#![feature(box_syntax, custom_derive, plugin, append, split_off)]
#![plugin(serde_macros)]

extern crate hyper;
extern crate url;
extern crate chrono;
extern crate serde;
extern crate serde_json;

pub mod dropbox;
pub mod metadata;
pub mod error;

#[cfg(test)]
mod tests {

    use std::io;
    use std::io::{Write, Read};
    use dropbox::DBClient;
    use metadata::MetaData;

    #[test]
    fn test_new() {
        DBClient::new("", "");
    }

    #[test]
    fn test_sample_file_put() {
        let f = b"Hello world!";
        let token = include_str!("oauth_token").to_string();
        let dbc = DBClient::new_with_token("", "", token);
        let mut dbf = dbc.get_file("test1.txt".to_string());
        dbf.write_all(f).unwrap();
    }

    #[test]
    fn test_sample_file_get() {
        let f = b"Hello world!";
        let token = include_str!("oauth_token").to_string();
        let dbc = DBClient::new_with_token("", "", token);
        let mut dbf = dbc.get_file("test1.txt".to_string());
        let mut rf: Vec<u8> = Vec::with_capacity(f.len());
        dbf.read_to_end(&mut rf).unwrap();
/*        for (a, b) in f.iter().zip(rf) {
            assert_eq!(*a, b);
        }*/
        for i in 0..f.len() {
            assert_eq!(f[i], rf[i]);
        }
    }

    #[test]
    fn test_sample_metadata() {
        let f = b"Hello world!";
        let token = include_str!("oauth_token").to_string();
        let dbc = DBClient::new_with_token("", "", token);
        let mut dbf = dbc.get_file("test1.txt".to_string());
        dbf.write_all(f).unwrap();

        let meta: MetaData = match dbf.lastmsg {
            Some(s) => s,
            None => panic!("No metadata found"),
        };
        assert_eq!(&meta.thumb_exists, &false);
        assert_eq!(&meta.bytes, &f.len());
    }

    #[test]
    fn test_oauth2_flow() {
        let client_id = include_str!("client_id");
        let client_secret = include_str!("client_secret");
        let mut dbc = DBClient::new(client_id, client_secret);
        println!("Go to: {}\nEnter code:",
                 dbc.get_authorize_url());

        let mut code = String::new();
        io::stdin().read_line(&mut code)
            .ok()
            .expect("Failed to read code line");
        println!("Token: {}", dbc.set_token(&code).unwrap());
    }
}



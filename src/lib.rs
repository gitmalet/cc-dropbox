extern crate hyper;

pub mod dropbox;

#[cfg(test)]
mod tests {

    use std::io::{Write, Read};
    use dropbox::DBClient;

    #[test]
    fn test_new() {
        DBClient::new();
    }

    #[test]
    fn test_sample_file_put() {
        let f = b"Hello world!";
        let token = include_str!("oauth_token").to_string();
        let dbc = DBClient::new_with_token(token);
        let mut dbf = dbc.get_file("test1.txt".to_string());
        dbf.write_all(f).unwrap();
    }

    #[test]
    fn test_sample_file_get() {
        let f = b"Hello world!";
        let token = include_str!("oauth_token").to_string();
        let dbc = DBClient::new_with_token(token);
        let mut dbf = dbc.get_file("test1.txt".to_string());
        let mut rf: Vec<u8> = Vec::with_capacity(f.len());
        dbf.read_to_end(&mut rf).unwrap();
        for (a, b) in f.iter().zip(rf) {
            assert_eq!(*a, b);
        }
    }
}



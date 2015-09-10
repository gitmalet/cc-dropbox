extern crate hyper;

pub mod dropbox;

#[cfg(test)]
mod tests {

    use std::io::Write;
    use dropbox::DBClient;

    #[test]
    fn test_new() {
        DBClient::new();
    }

    #[test]
    fn test_sample_file() {
        let f = b"Hello world!";
        //TODO: DELETE BEFORE PUBLISH
        let token = include_str!("oauth_token").to_string();
        let dbc = DBClient::new_with_token(token);
        let mut dbf = dbc.get_file("test1.txt".to_string());
        dbf.write_all(f).unwrap();
    }
}



pub mod settings{
use crate::args::args::*;
    pub struct Settings{
        base_uri : String,
    }
    impl Settings{
        pub fn new() -> Self{
            Self { base_uri : "https://wallhaven.cc/api/v1/search".to_string() }
        }
        pub fn get_uri (&self, args : &Args) -> Result<String, Box<dyn std::error::Error>>{
            let options : String = args.build_request()?;
            Ok(format!("{}{}", self.base_uri, options))
        }
    }
}

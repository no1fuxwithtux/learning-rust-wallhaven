pub mod response{
    use serde::Deserialize;
    use std::error;
    use std::io::Write;
    use std::fs::File;

    #[derive(Deserialize)]
    pub struct Thumb {
        pub large : String,
        pub original : String,
        pub small : String
    }
    #[derive(Deserialize)]
    pub struct Data {
        pub id : String,
        pub url : String,
        pub short_url : String,
        pub views: u32,
        pub favorites : u32,
        pub purity : String,
        pub dimension_x : u32,
        pub dimension_y : u32,
        pub resolution : String,
        pub ratio : String,
        pub file_size : u32,
        pub file_type : String,
        pub created_at : String,
        pub colors : Vec<String>,
        pub path : String,
        pub thumbs : Thumb,
    }

    #[derive(Deserialize)]
    pub struct SearchResult {
        pub data : Vec<Data>
    }

    #[tokio::main]
    pub async fn submitwebrequest(uri : &str) -> Result<SearchResult, Box<dyn error::Error>> {
        let response = reqwest::get(uri).await?;
        let body = response.text().await?;
        //Now we can convert the actual body to json
        let result_object = serde_json::from_str(&body)?; 
        Ok(result_object)
    }
    #[tokio::main]
    pub async fn downloadwallhavenpicture (pictureinfo : &Data) -> Result<String, Box<dyn error::Error>> {
        //let tmp_dir = Builder::new().prefix("wallhaven").tempdir()?;
        let target = &pictureinfo.path; 
        let response = reqwest::get(target).await?;
        let pathname : String;
        let mut dest = {
            let fname = response 
                .url()
                .path_segments()
                .and_then(|segments| segments.last())
                .and_then(|name| if name.is_empty() { None } else { Some(name)} )
                .unwrap_or("tmp.bin").to_string();
            pathname = format!("/tmp/{}", &fname);
            println!("creating file under {}", &pathname);
            File::create(&pathname)?
        };
        let content = response.bytes().await?;
        dest.write_all(&content)?;
        Ok(pathname)
    }
}

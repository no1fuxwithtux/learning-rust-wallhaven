pub mod args {
    use std::fmt::{self};
    //more efficient than a vector for arg operations
    use std::collections::VecDeque;

    #[derive(Debug)]
    pub struct ArgName {
        pub arg_name_long : Option<String>,
        pub arg_name_short : Option<String>,
    }

    #[derive(Debug,)]
    pub struct ArgError {
        pub error: String, 
    }
    impl std::error::Error for ArgError{

    }
    impl fmt::Display for ArgError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.error)
        }
    }

    pub trait NewArg<T> {
        fn new (input : &str) -> Result<T, ArgError>;
    }

    pub enum Category{
        General,
        Anime,
        People
    }
    pub enum Purity  {
        Sfw,
        Sketchy,
        Nsfw
    }
    pub enum Sorting {
        DateAdded,
        Relevance,
        Random,
        Views,
        Favorites,
        Toplist
    }
    pub enum Order {
        Desc,
        Asc
    }
    pub enum TopRange{
        OneDay,
        ThreeDays,
        OneWeek,
        OneMonth,
        ThreeMonths,
        SixMonths,
        OneYear, 
    }
    pub struct Resolution{
        pub height : u16,
        pub width : u16,
    }
    pub struct MinResolution {
        pub height: u16,
        pub width : u16
    }
    pub struct Ratio {
        pub width : u8,
        pub height : u8,
    }
    pub struct Color{
        hex_value : String,
    }
    pub struct Seed{
        seed_value : String,
    }
    pub enum TagModifier{
        Plus,
        Minus
    }
    pub struct Tag {
        pub tagname : String,
        //fuzzy search does not require a tag
        pub tagmodifier : Option<TagModifier>,
    }
    pub enum Arg {
        Tag (VecDeque<Tag>),
        Categories (VecDeque <Category>),
        Purity (VecDeque <Purity>),
        Sorting (Option<Sorting>),
        Order (Option<Order>),
        TopRange (Option<TopRange>),
        AtLeast (Option<MinResolution>),
        Resolutions (VecDeque<Resolution>),
        Ratios (VecDeque<Ratio>),
        Colors (VecDeque<Color>),
        Page (Option<u32>),
        Seed (Option<Seed>),
        ApiKey (Option<String>),
        DownloadDir (Option<String>),
        Command (Option<String>),
        StdOut,
    }

    pub struct Args<> {
        pub args : VecDeque<Arg>,
    }
    impl Args {
        pub fn parse_input (&mut self, input : &str) -> Result<(), Box<dyn std::error::Error>> {
            match Args::get_string_mapping(&input) {
                Ok(result) => { self.args.push_front(result); Ok(()) },
                Err(_) => {
                    if let Some(arg) = self.args.front_mut(){
                        match arg {
                            Arg::Tag(vec) => { let tag = Tag::new(input)?; vec.push_front(tag); },
                            Arg::Categories(vec) => { let category = Category::new(input)?; vec.push_front(category);},
                            Arg::Purity(vec) => { let purity = Purity::new(input)?; vec.push_front(purity);},
                            Arg::Sorting(val) => { let sorting = Sorting::new(input)?; *val = Some(sorting); },
                            Arg::Order(val) => { let order = Order::new(input)?; *val = Some(order);},
                            Arg::TopRange(val) => { let toprange = TopRange::new(input)?; *val = Some(toprange);},
                            Arg::AtLeast(val) => { let atleast = MinResolution::new(input)?; *val = Some(atleast);},
                            Arg::Resolutions(vec) => { let res = Resolution::new(input)?; vec.push_front(res);},
                            Arg::Ratios(vec) => { let ratio = Ratio::new(input)?; vec.push_front(ratio);},
                            Arg::Colors(vec) => { let color = Color::new(input)?; vec.push_front(color);},
                            Arg::Page(val) => { let page = input.parse::<u32>()?; *val = Some(page);},
                            Arg::Seed(val) => { let seed = Seed::new(input)?; *val = Some(seed);},
                            Arg::ApiKey(val) => { *val = Some(String::from(input))},
                            Arg::DownloadDir(val) => { *val = Some(String::from(input))},
                            Arg::Command(val) => { *val = Some(String::from(input))},
                            _ => { Err(ArgError { error : format!("unsupported option: {}\nsupported options:\n{}", input, self.print_options()) })?; }
                        }
                        Ok(())
                    }
                    else{
                        Err(ArgError { error: format!("unsupported option: {}\nsupported options:\n{}", input, self.print_options()) })?
                    }
                } 
            }
        }

        pub fn print_options(&self) -> String {
            format!("-t, --tag: Tag for the search query\n-c, --categories: Category for the search (General, People, Anime)\n-p, --purity: Purity for the search (Sfw,Sketchy,Nsfw)\n-s, --sorting: Sorting method (DateAdded, Relevance, Random, Views, Favorites, Toplist)\n--order: Asc Desc\n--atleast: Resolution seperated by x\n--resolutions: List of resolutions to search for\n--ratios: List of ratios with x seperator\n--colors: List of colors in Hex-format\n--page: page to query from results (probably useless right now)\n--seed: specific seed value for the search\n--apikey: required for Nsfw images")
        }

        pub fn get_string_mapping(input : &str) -> Result<Arg, ArgError> {
            match input {
                "-t" => Ok(Arg::Tag(VecDeque::new())),
                "--tag" => Ok(Arg::Tag(VecDeque::new())),
                "-c" => Ok(Arg::Categories(VecDeque::new())),
                "--categories" => Ok(Arg::Categories(VecDeque::new())),
                "-p" => Ok(Arg::Purity(VecDeque::new())),
                "--purity" => Ok(Arg::Purity(VecDeque::new())),
                "-s" => Ok(Arg::Sorting(None)),
                "--sorting" => Ok(Arg::Sorting(None)),
                "-o" => Ok(Arg::Order(None)),
                "--order" => Ok(Arg::Order(None)),
                "--toprange" => Ok(Arg::TopRange(None)),
                "--atleast" => Ok(Arg::AtLeast(None)),
                "--resolutions" => Ok(Arg::Resolutions(VecDeque::new())),
                "--ratios" => Ok(Arg::Ratios(VecDeque::new())),
                "--colors" => Ok(Arg::Colors(VecDeque::new())),
                "--page" => Ok(Arg::Page(None)),
                "--seed" => Ok(Arg::Seed(None)),
                "--apikey" => Ok(Arg::ApiKey(None)),
                "-d" => Ok(Arg::DownloadDir(None)),
                "--downloaddir" => Ok(Arg::DownloadDir(None)),
                "--command" => Ok(Arg::Command(None)),
                "--stdout" => Ok(Arg::StdOut),
                //We actually never read this error, but I still included it for completeness
                _ => Err(ArgError { error: String::from("no mapping found")}) 
            }
        }
        pub fn build_request (&self) -> Result<String, ArgError> {
            let mut result : String = String::from("?"); 
            for arg in &self.args {
                if result.chars().count() > 1 { 
                    result += "&";
                }
                match arg {
                    Arg::Tag(vec) => {
                        if vec.is_empty() {
                            return Err(ArgError { error : String::from("Tag defined, but no value provided!")});
                        }
                        else{
                            result+= "q=";
                            for tag in vec.iter() {
                                result += &format!("{},", tag);
                            }
                            result.pop();
                        }
                    },
                    Arg::Categories(vec) => {
                        if vec.len() == 0 {
                            return Err(ArgError { error: String::from("No value provided for categories!")});
                        }
                        else {
                            let mut bin : u8 = 0;
                            for cat in vec.iter() {
                                bin += cat.get_val();
                            }
                            result+= &format!("categories={:03b}", bin);                        
                        }
                    },
                    Arg::Purity(vec) => {
                        if vec.len() == 0 {
                            return Err(ArgError { error: String ::from("No value provided for purity!")});
                        }
                        else{
                            let mut bin : u8 = 0;
                            for cat in vec.iter() {
                                bin += cat.get_val();
                            }
                            result += &format!("purity={:03b}", bin)
                        }
                    },
                    Arg::Sorting(val) => {
                        if let Some(set_value) = val {
                            result+= "sorting="; 
                            result+= set_value.get_val();
                        }
                        else{
                            return Err(ArgError { error: String::from("No value provided for sorting!")});
                        }
                    },
                    Arg::Order(val) => {
                        if let Some(set_value) = val {
                            result += "order=";
                            result += set_value.get_val();
                        }
                        else{
                            return Err(ArgError { error: String::from("No value provided for order!")});
                        }
                    },
                    Arg::TopRange(val) => {
                        if let Some(set_value) = val {
                            result += "toprange=";
                            result += set_value.get_val();
                        }
                    },
                    Arg::AtLeast(val) => {
                        if let Some(set_value) = val {
                            result += &format!("atleast={}", set_value);
                        }
                    },
                    Arg::Resolutions(vec) => {
                        if vec.is_empty() {
                            return Err(ArgError { error : String::from("Resolutions defined, but no value provided!")});
                        }
                        else{
                            result+="resolutions=";
                            for res in vec.iter() {
                                result+= &format!("{},", res);
                            }
                            result.pop();
                        }
                    },
                    Arg::Ratios(vec) => {
                        if vec.is_empty() {
                            return Err(ArgError { error: String::from("Ratios defined, but no value provided!")});
                        }
                        else{
                            result+="ratios=";
                            for rat in vec.iter() {
                                result+= &format!("{},", rat);
                            }
                            result.pop();
                        }
                    },
                    Arg::Colors(vec) => {
                        if vec.len() == 0 {
                            return Err(ArgError { error: String::from("Colors defined, but no value provided!")});
                        }
                        else{
                            result+="colors=";
                            for col in vec.iter() {
                                result+= &format!("{},", col);
                            }
                            result.pop();
                        }
                    },
                    Arg::Page(val) => {
                        if let Some(set_value) = val {
                            result+= "page=";
                            result+= &set_value.to_string();
                        }
                        else{
                            return Err(ArgError { error: String::from("Page defined, but no page given!")});
                        }
                    },
                    Arg::Seed(val) => {
                        if let Some(set_value) = val {
                            result+=&format!("seed={}", set_value.seed_value);
                        }
                    },
                    Arg::ApiKey(val) => {
                        if let Some(set_value) = val {
                            result+=&format!("apikey={}", set_value);
                        }
                    },
                    //One Option not covered by the api itself is the downloaddir option this cannot
                    //be declared here since it does not have a direct influence on the resulting
                    //url
                    _ => ()
                }
            }
            Ok(result)
        }    
    }

    impl Category {
        pub fn get_val(&self) -> u8 {
            match self {
                Category::General => 4,
                Category::Anime => 2,
                Category::People => 1,
            }
        }
    }

    impl NewArg<Category> for Category {
        fn new (input : &str) -> Result<Category, ArgError>{
            match input.to_uppercase().as_ref() {
                "GENERAL" => Ok(Category::General),
                "ANIME" => Ok(Category::Anime),
                "PEOPLE" => Ok(Category::People),
                _ => Err(ArgError { error: String::from("unknown category (valid: General, Anime, People)")})
            }
        }
    }

    impl Purity {  
        pub fn get_val(&self) -> u8 {
            match self {
                Purity::Sfw => 4,
                Purity::Sketchy => 2,
                Purity::Nsfw => 1
            }
        }
    }

    impl NewArg<Purity> for Purity {
        fn new (input : &str) -> Result<Purity, ArgError> {
            match input.to_uppercase().as_ref() {
                "NSFW" => Ok(Purity::Nsfw),
                "SKETCHY" => Ok(Purity::Sketchy),
                "SFW" => Ok(Purity::Sfw),
                _ => Err(ArgError { error: String::from("unkown purity (valid: Sfw, Sketchy, Nsfw)")})
            }
        }
    }


    impl TopRange {
        pub fn get_val(&self) -> &str {
            match self {
                TopRange::OneDay => "1d",
                TopRange::ThreeDays => "3d",
                TopRange::OneWeek => "1w",
                TopRange::OneMonth => "1M",
                TopRange::ThreeMonths => "3M",
                TopRange::SixMonths => "6M",
                TopRange::OneYear => "1y"
            }
        }
    }

    impl NewArg<TopRange> for TopRange {
        fn new (input : &str) -> Result<TopRange, ArgError> {
            match input.to_uppercase().as_ref() {
                "1D" => Ok(TopRange::OneDay),
                "3D" => Ok(TopRange::ThreeDays),
                "1W" => Ok(TopRange::OneWeek),
                "1M" => Ok(TopRange::OneMonth),
                "3M" => Ok(TopRange::ThreeMonths),
                "6M" => Ok(TopRange::SixMonths),
                "1Y" => Ok(TopRange::OneYear),
                _ => Err(ArgError { error: String::from("unknown top range")})
            }
        }
    }

    impl Sorting {
        pub fn get_val(&self) -> &str {
            match self {
                Sorting::DateAdded => "date_added",
                Sorting::Relevance => "relevance",
                Sorting::Random => "random",
                Sorting::Views => "views",
                Sorting::Favorites => "favorites",
                Sorting::Toplist => "toplist",
            }
        }
    }

    impl NewArg<Sorting> for Sorting {
        fn new (input : &str) -> Result<Sorting, ArgError> {
            match input.to_uppercase().as_ref() {
                "DATEADDED" => Ok(Sorting::DateAdded),
                "RELEVANCE" => Ok(Sorting::Relevance),
                "RANDOM" => Ok(Sorting::Random),
                "VIEWS" => Ok(Sorting::Views),
                "FAVORITES" => Ok(Sorting::Favorites),
                _ => Err (ArgError { error: String::from("unknown sorting method (valid: DateAdded, Relevance, Random, Views, Favorites)")})
            }
        }
    }

    impl Order {
        pub fn get_val(&self) -> &str{
            match self{
                Order::Asc => "asc",
                Order::Desc => "desc"
            }
        }
    }

    impl NewArg<Order> for Order {
        fn new (input : &str) -> Result<Order, ArgError> {
            match input {
                "ASC" => Ok(Order::Asc),
                "DESC" => Ok(Order::Desc),
                _ => Err (ArgError { error: String::from("unknown order type")})
            }
        }
    }

    impl fmt::Display for Resolution {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}x{}", self.width, self.height )           
        }
    }

    impl NewArg<Resolution> for Resolution {
        fn new (input : &str) -> Result<Resolution, ArgError> {
            if input.contains("x") {
                let mut i : u8 = 0;
                let mut width = 0;
                let mut height = 0;
                for item in input.split(|item : char | item == 'x') {
                    if item.len() > 0 {
                        if let Ok(val) = item.parse::<u16>() {
                            if i < 1 {
                                width = val;
                            }
                            else {
                                height = val;
                            }
                            i+=1;
                        }
                        else{
                            return Err (ArgError { error: String::from("value for width or height missing or invalid")})
                        }
                    }
                    else {
                        return Err (ArgError { error: String::from("value for width or height missing or invalid")})
                    }
                }
                Ok(Resolution {width: width, height : height})
            }
            else {
                Err (ArgError { error: String::from("invalid value for resolution not x seperator!")})
            }
        }
    }

    impl NewArg<MinResolution> for MinResolution {
        fn new (input : &str) -> Result<MinResolution, ArgError> {
            if input.contains("x") {
                let mut i : u8 = 0;
                let mut width = 0;
                let mut height = 0;
                for item in input.split(|item : char | item == 'x') {
                    if item.len() > 0 {
                        if let Ok(val) = item.parse::<u16>() {
                            if i < 1 {
                                width = val;
                            }
                            else {
                                height = val;
                            }
                            i+=1;
                        }
                        else{
                            return Err (ArgError { error: String::from("value for width or height missing or invalid")})
                        }
                    }
                    else {
                        return Err (ArgError { error: String::from("value for width or height missing or invalid")})
                    }
                }
                Ok(MinResolution {width: width, height : height})
            }
            else {
                Err (ArgError { error: String::from("invalid value for resolution no x seperator!")})
            }
        }
    }


    impl fmt::Display for MinResolution {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}x{}", self.width, self.height )           
        }
    }

    impl fmt::Display for Ratio {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
            write!(f, "{}x{}", self.width, self.height)
        }
    }
    impl NewArg<Ratio> for Ratio {
        fn new (input : &str) -> Result<Ratio, ArgError> {
            if input.contains("x"){
                let mut i = 0;
                let mut width : u8 = 0;
                let mut height : u8 = 0; 
                for item in input.split(|item| item == 'x'){
                    if item.len() > 0 {
                        if let Ok(val) = item.parse::<u8>(){
                            if i < 1 {
                                width = val;
                            }
                            else{
                                height = val;
                            }
                            i+=1;
                        }
                        else{
                            return Err::<Ratio,ArgError>(ArgError {error: "value for width or height in ratio invalid!".to_string() });
                        }
                    }
                    i +=1;
                }
                Ok(Ratio {width: width, height: height })
            }
            else {
                Err (ArgError { error : String::from("invalid value for ratio no x seperator")})
            }

        }
    } 
    impl NewArg<Color> for Color {
        fn new (hex : &str) -> Result<Color,ArgError> {
            let mut missmatch : bool = false;
            let result_hex = hex.replace('#', "").to_lowercase();
            let valid_characters : [char;15] = [ '1','2','3','4','5','6','7','8', '9', 'a','b','c','d','e','f'];
            if ! result_hex.chars().count() == 6 && result_hex.chars().count() == 8 {
                missmatch = true;
            }
            if ! missmatch {
                let _= result_hex.chars().into_iter().map(|item| 
                    {
                        if ! valid_characters.contains(&item) {
                            missmatch = false;
                        }
                    });
            }
            if missmatch {
                Err(ArgError { error : String::from("invalid value for color")})
            }
            else{
                Ok (Self { hex_value : hex.to_string()})
            }
        }
    } 
    impl fmt::Display for TagModifier{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let modifier : String;
            match self{
                TagModifier::Plus => modifier = "+".to_string(),
                TagModifier::Minus => modifier = "-".to_string()
            }
            write!(f, "{}", modifier)
        }
    }
    impl fmt::Display for Tag{
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.tagmodifier.is_some(){
                write!(f, "{}{}", self.tagmodifier.as_ref().unwrap(), self.tagname)
            }
            else {
                write!(f, "{}", self.tagname)
            }
        }
    }
    impl NewArg<Tag> for Tag {
        fn new (input : &str) -> Result<Tag, ArgError> {
            if input.len() > 1{
                if input.contains("+"){
                    Ok(Tag {tagname : input.replace("+", ""), tagmodifier: Some(TagModifier::Plus)})
                }
                else if input.contains("-"){
                    Ok(Tag { tagname: input.replace("-", ""), tagmodifier: Some(TagModifier::Minus)})
                }
                else{
                    Ok(Tag {tagname: input.to_string(), tagmodifier: None})
                }
            }
            else{
                Err(ArgError { error : String::from("invalid tag declaration")})
            }
        }
    }
    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.hex_value.to_lowercase())
        }
    }
    impl NewArg<Seed> for Seed {
        fn new (seed : &str) -> Result<Seed, ArgError> {
            let mut missmatch : bool = false;
            let valid_characters : [char; 36] =  ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o','p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',];
            let _ = seed.chars().into_iter().map(|item|{
                if valid_characters.contains(&item){
                    missmatch = true;
                }
            });
            if missmatch 
            {
                Err(ArgError { error : String::from("Invalid Characters in Seed")})
            }
            else{
                Ok(Seed{ seed_value : seed.to_string() })
            }
        }
    }
}


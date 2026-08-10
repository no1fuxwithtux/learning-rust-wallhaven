pub mod args;
pub mod settings;
pub mod response;
use crate::args::args::*;
use std::collections::VecDeque;
use std::env;
use crate::settings::settings::*;
use std::process::Command;
fn main() {
    //In order to actually make this somewhat useful we have to accept args that allow us to mutate
    //the actual object one solution would be to have args that define how they write themselfes or
    //add a trait to the Args that allows us to define what string represents them. Yes that's the
    //best solution
    /*let arg_collection = Args{   
        categories: Some(vec![Category::Anime]), 
        purity: Some(vec![Purity::Sfw]),
        sorting: Some(Sorting::Random),
        apikey : "".to_string(), ..Default::default() };
    */
    //first argument is always the programm name and not in our interest
    if let Ok(cliargs) = cli_arg_parse(&env::args().skip(1).collect()) {
        if let Ok(uri) = Settings::new().get_uri(&cliargs) {
        let result = response::response::submitwebrequest(&uri);
        match result {
        Ok(object) => {
            if let Some(data) = object.data.first() {
                let mut download_dir : Option<String> = None;
                for arg in cliargs.args.iter() {
                    match arg {
                        Arg::DownloadDir(val) => { if let Some(final_val) = val { download_dir = Some(final_val.to_string())} else { download_dir = None}; break; },
                        _ => (),
                    }

                }
                let download_result = response::response::downloadwallhavenpicture(&data, &download_dir);
                match download_result {
                    Ok(download_result_string) => { 
                        println!("{}", download_result_string);
                        Command::new("feh")
                                .arg(&download_result_string)
                                .output()
                                .expect("failed to execute process");
                       
                    },
                    Err(err) => {
                        println!("{}", err)
                    }
                }
            }
        },
        Err(err) => {
            println!("{}", err);
        }
    }
        }
    }
}

fn cli_arg_parse (args: &Vec<String>) -> Result<Args, Box<dyn std::error::Error>> {
    let mut cli_args = Args { args: VecDeque::new() };
        for arg in args{
            cli_args.parse_input(arg)?
        }
    Ok(cli_args)
}


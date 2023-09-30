mod database;
mod req;

use std::io::stdout;
use std::{fs, io};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::{PathBuf};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    ExecutableCommand,
    event,
};
use faster_hex::hex_decode;
use glob::{glob, Paths};

use indicatif::{ProgressBar, ProgressStyle};
use serde_json::Value;
use terminal_emoji::Emoji;
use crate::database::{ModDataBase, ModsList};
use crate::req::{request_get};


fn main() {
    execute!(
        std::io::stdout(),
        crossterm::terminal::SetTitle("Minecraft translated mods downloader")
    );

    let sty = ProgressStyle::with_template(
        "[{elapsed_precise}] {prefix:.bold.dim} {spinner:.green}\n[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} [{msg}]",

    )
    .unwrap()
    .progress_chars("##-")
    .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    // Getting list
    let database: Result<ModsList, String> = ModDataBase::get_list();

    let mut mods: HashMap<String, String> = HashMap::new();

    match database {
        Ok(bd) => {
            // Mod checking
            let mut vec: Vec<PathBuf> = vec![];

            let files: Paths = glob("mods//*.jar").unwrap();

            for path in files {
                let p = path.unwrap();
                vec.push(p);
            }

            let count = vec.clone().iter().count();

            if count > 0 {
                let pb = ProgressBar::new(count as u64);
                pb.set_style(sty.clone());
                pb.set_prefix("Checking mods");

                let mut last_index = 0;

                for index in 0..vec.len() {
                    let path = vec.get(index).unwrap();
                    let path2 = path.clone();

                    let bytes = fs::read(path).unwrap();  // Vec<u8>
                    let hash = sha256::digest(&bytes);


                    let path_str = String::from(path2.as_path().as_os_str().to_str().unwrap());
                    let path_str_lenght = path_str.len();



                    pb.set_message(format!("{}", &path_str[5..path_str_lenght]));
                    if bd.lists.contains(&hash) {
                        mods.insert(hash, path_str);
                        last_index = index;


                    }
                    pb.inc(1);
                }

                pb.set_message(format!("{}", "done"));
                pb.finish();
            }
            else {
                println!("Not founded minecraft mods directory");

            }


            let mods_count = mods.iter().count();
            if mods_count > 0 {

                let pb2 = ProgressBar::new(mods_count as u64);
                pb2.set_style(sty.clone());
                println!("Translation of the mods...");
                for mods_dict in mods {
                    match request_get(format!("https://github.com/Minecraft-mod-translations/Cloud/raw/main/files/{}.json", mods_dict.0).as_str()) {
                        None => { println!("Error translation mod: {}", mods_dict.1) }
                        Some(body) => {
                            match serde_json::from_str::<Value>(body.as_str()) {
                                Ok(json) => {
                                    let hex_string_option = json["file"].as_str();
                                    let name_option = json["name"].as_str();

                                    match name_option {
                                        None => {}
                                        Some(name) => { pb2.set_message(format!("{}", name));}
                                    }

                                    match hex_string_option {
                                        None => {
                                            println!("File not found")
                                        }
                                        Some(hex_string) => {
                                            let bytes = string_to_hex(hex_string.to_string());

                                            let path_normal = mods_dict.1;

                                            let remove = fs::remove_file(path_normal.clone());
                                            match remove {
                                                Ok(_) => {
                                                    let mut file = fs::OpenOptions::new()
                                                        .create(true) // To create a new file
                                                        .write(true)
                                                        // either use the ? operator or unwrap since it returns a Result
                                                        .open(format!("{}", path_normal));
                                                    match file {
                                                        Ok(mut file_result) => {
                                                            let write_status = file_result.write_all(&*bytes);
                                                            match write_status {
                                                                Ok(_) => {}
                                                                Err(_) => { println!("Error write file") }
                                                            }
                                                        }
                                                        Err(_) => { println!("Error file not opened file") }
                                                    }
                                                }
                                                Err(_) => { println!("Error removing file: {}", path_normal.as_str()) }
                                            }
                                            pb2.inc(1);
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Conversions to Json error: {}", e)
                                }
                            }
                        }
                    }
                }
                pb2.set_message(format!("{}", "done"));
                pb2.finish();
            }
            else {
                println!("No translation mods found")
            }
        }
        Err(error) => {}
    }
    io::stdin().read_line(&mut String::new()).unwrap();
}

pub fn string_to_hex(str: String) -> Vec<u8> {
    let href: &[u8]= str.as_ref();
    let mut dst = vec![0; href.len() / 2];
    hex_decode(href, &mut dst).unwrap();
    dst
}

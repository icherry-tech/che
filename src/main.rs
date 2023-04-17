use std::{
    env,
    fs::{self, File},
    io::Write,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "che")]
struct Cli {
    /// 目录名
    #[structopt(short = "d", long)]
    dir: Option<String>,
    /// 文件名
    #[structopt(short = "n", long)]
    name: String,
}
fn main() {
    let args = Cli::from_args();
    let default_dir = ["router", "handler", "service", "dao"];
    let home_path = env::current_dir().unwrap();
    for i in default_dir {
        let mut path = home_path.join(i.to_string());
        let mut package_name = i.to_string();
        if !args.dir.is_none() {
            let temp = args.dir.clone();
            let dir = format!("{}", temp.unwrap());
            package_name = dir.clone();
            path = path.join(dir);
        }
        let bible_dir_path = match path.to_str() {
            None => "0",
            Some(s) => s,
        };
        fs::create_dir_all(&bible_dir_path).unwrap();
        let file_name = path.join(args.name.to_string() + ".go");
        let file_name_string = match file_name.to_str() {
            None => "0",
            Some(s) => s,
        };
        let data = check_file(file_name_string);
        if data == 0 {
            let mut f = File::create(file_name).unwrap();
            let txt = format!("package {}", package_name);
            f.write_all(txt.as_bytes()).unwrap();
        }
    }
}

fn check_file(bible_path: &str) -> i32 {
    let f = File::open(bible_path);
    let result = match f {
        Ok(_file) => 1,
        Err(_err) => 0,
    };
    result
}

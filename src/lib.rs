use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn get_link() -> Result<String, io::Error> {
    // 首先获取用户输入的网址
    println!("");
    println!("");
    println!("请关留守儿童和孤寡老人,如果您有机会遇到他们,请务必善待他们.");
    println!("作者QQ:756423901");
    println!("");
    println!("");
    println!("请输入IPFS根地址:");
    let mut url = String::new();
    std::io::stdin().read_line(&mut url)?;
    Ok(url)
}
pub fn analysis(code: &str) -> Vec<&str> {
    let mut file_names: Vec<&str> = vec![];
    let mut code = &code[..];
    loop {
        if let Some(begin) = code.find("filename=") {
            code = &code[begin..];
        } else {
            break;
        }
        if let Some(begin) = code.find("\"") {
            file_names.push(&code[9..begin]);
            //println!("{}{}", url, &code[9..begin]);
            code = &code[begin..]
        } else {
            break;
        }
    }
    file_names
}
pub fn make_dir(hash: &str) -> Result<PathBuf, io::Error> {
    //创建文件夹
    let dir = std::env::current_dir().unwrap();
    let dir = dir.join(hash);
    let dir_path = Path::new(&dir);
    if dir.is_dir() {
        fs::remove_dir_all(dir_path)?;
    }
    fs::create_dir(dir_path)?;
    println!("所有输出将保存在以下目录:{:?}", dir_path);
    Ok(dir)
}

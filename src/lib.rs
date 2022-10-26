use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub fn get_input() -> Result<(String, i32), Box<dyn std::error::Error>> {
    // 首先获取用户输入的网址
    println!("");
    println!("");
    println!("请关留守儿童和孤寡老人,如果您有机会遇到他们,请务必善待他们.");
    println!("作者QQ:756423901");
    println!("");
    println!("");
    // 获取根地址
    println!("请输入IPFS根地址:");
    let mut url = String::new();
    std::io::stdin().read_line(&mut url)?;
    // 获取线程数
    let thread_num: i32 = loop {
        let mut thread_num = String::new();
        println!("请设置下载线程数:");
        std::io::stdin().read_line(&mut thread_num)?;
        match thread_num.trim().parse() {
            Ok(num) => break num,
            _ => (),
        }
    };

    Ok((url, thread_num))
}

pub fn analysis(code: &str) -> Vec<String> {
    let mut file_names: Vec<String> = vec![];
    let mut code = &code[..];
    loop {
        if let Some(begin) = code.find("filename=") {
            code = &code[begin..];
        } else {
            break;
        }
        if let Some(begin) = code.find("\"") {
            file_names.push((&code[9..begin]).to_string());
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
    println!("{:?}", dir_path);
    if dir.is_dir() {
        fs::remove_dir_all(dir_path)?;
    }
    fs::create_dir(dir_path)?;
    println!("所有输出将保存在以下目录:{:?}", dir_path);
    Ok(dir)
}
pub fn download(url: &str, file_path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("{url}");
    let body = reqwest::blocking::get(url)?.bytes()?;
    let mut file = match std::fs::File::create(file_path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content = body.bytes();
    let data: std::result::Result<Vec<_>, _> = content.collect();
    file.write_all(&data.unwrap())?;
    Ok(())
}
pub fn get_code(url: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let code = reqwest::blocking::get(url)?.text()?;
    Ok(code)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_download() {
        let url = "https://ipfs.io/ipfs/bafybeiabzw76fnrb5ca56zz5xtsybcgj4hx2v66a4whuvwjxaitmgprndq/7520.png";
        let _ = download(url, "./7520.png");
        let file = Path::new("./7520.png");
        assert!(file.is_file());
    }
}

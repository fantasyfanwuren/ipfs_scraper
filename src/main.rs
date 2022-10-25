use std::io::{Read, Write};
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 首先获取用户输入的网址
    let url = ipfs_scraper::get_link()?;
    let url = url.trim();
    // 根据网址获取网页源码
    let client = reqwest::Client::new();
    let code = client.get(url).send().await?.text().await?;

    //解析源码
    let file_names = ipfs_scraper::analysis(&code);
    // 创建文件夹(文件名选取链接的10-20之间的字符,这是哈希值的一部分)
    let dir_path = ipfs_scraper::make_dir(&url[10..20])?;
    //单线程逐个下载
    for file_name in file_names {
        println!("{}:开始下载...", file_name);
        let url = url.to_string() + file_name;
        let body = client.get(&url).send().await?.bytes().await?;

        let path = dir_path.join(file_name);
        let mut file = match std::fs::File::create(&path) {
            Err(why) => panic!("couldn't create {}", why),
            Ok(file) => file,
        };
        let content = body.bytes();
        let data: std::result::Result<Vec<_>, _> = content.collect();
        file.write_all(&data.unwrap())?;
        println!("{}:下载完成!", file_name);
    }
    Ok(())
}

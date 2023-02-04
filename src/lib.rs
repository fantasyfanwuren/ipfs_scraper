use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// # 该函数用于获取用户的输入
///
/// 首先获取用户输入的网址
///
/// 然后获取用户输入的线程数
///
pub fn get_input() -> Result<(String, i32), Box<dyn std::error::Error>> {
    // 首先获取用户输入的网址
    println!("***************************************************************");
    println!("*鸣谢:Playthings by JIA 玩物之佳(WX)对本开源项目的<创始性>捐赠*");
    println!("************鸣谢:徐龙先生对本开源项目的<升级性>捐赠!***********");
    println!("**请关留守儿童和孤寡老人,如果您有机会遇到他们,请务必善待他们.**");
    println!("***********************作者QQ:756423901************************");
    println!("***************************************************************");

    // 获取根地址
    println!("请输入IPFS根哈希:");
    let mut url = String::new();
    std::io::stdin().read_line(&mut url)?;
    // 获取线程数
    let thread_num: i32 = loop {
        let mut thread_num = String::new();
        println!("请设置下载线程数:(推荐50-500个线程同步下载)");
        std::io::stdin().read_line(&mut thread_num)?;
        match thread_num.trim().parse() {
            Ok(num) => break num,
            _ => (),
        }
    };

    Ok((url, thread_num))
}

/// # 解析从ipfs网站获取的源码信息,从中获取所有需要下载的文件名
///
/// 它首先会查找第一个"filename="字符串,然后找到紧随其后的"\""(引号)
///
/// 然后获取 两个字符串之间的内容就是文件名
///
/// 循环处理以上内容,得到一个只含有文件名的数组Vec<String>
///
pub fn analysis(code: &str) -> Vec<String> {
    let mut file_names: Vec<String> = vec![];
    let mut code = &code[..];
    loop {
        // 首先找到第一个"filename="的位置 begin
        if let Some(begin) = code.find("filename=") {
            // 将切片指向begin之后的位置
            code = &code[begin..];
        } else {
            // 若无法找到,说明链接有问题,或者已经循环完毕,就退出
            break;
        }
        // 找到"filename="后面紧跟着的引号,将其位置定义为begin
        if let Some(begin) = code.find("\"") {
            // 截取 "filename=" 和 其后引号之间的文本,加入到要返回的Vec<String>中
            file_names.push((&code[9..begin]).to_string());
            // 将切片指向新的begin之后的内容
            code = &code[begin..]
        } else {
            break;
        }
    }
    file_names
}

/// # 该函数以一个字符串hash作为参数,然后在当前程序所在目录下创建一个hash字符串为命名的文件夹
///
/// 它内部会进行一定的判断:如果文件夹已经存在就把他删除,并重新创建,如果不存在就直接创建
///
pub fn make_dir(hash: &str) -> Result<PathBuf, io::Error> {
    //创建文件夹
    let dir = std::env::current_dir().unwrap();
    let dir = dir.join(hash);
    let dir_path = Path::new(&dir);
    // 判断字符串是否存在
    if dir.is_dir() {
        fs::remove_dir_all(dir_path)?;
    }
    fs::create_dir(dir_path)?;
    println!("所有输出将保存在以下目录:{:?}", dir_path);
    Ok(dir)
}
/// # 提供下载功能:参数为下载链接的地址和下载后需要保存的路径(这个路径是包含保存后的命名的)
///
/// 实现原理很简单:先下载数据,在把数据转化为可写入的类型(该类型内含错误传递机制)
/// 写入并保存
pub fn download(url: &str, file_path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 先下载数据
    let body = reqwest::blocking::get(url)?.bytes()?;
    // 创建一个空文件
    let mut file = match std::fs::File::create(file_path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    // 将下载后的数据转化为Bytes<&[u8]>类型
    let content = body.bytes();
    // 将数据Bytes<&[u8]>类型转化为 带有错误传递机制的Vec<[u8]>类型,这是可以写入文件的一种类型
    let data: std::result::Result<Vec<_>, _> = content.collect();
    // 将数据写入之前创建的空文件
    file.write_all(&data?)?;
    Ok(())
}
/// #从目标链接获取网页的源码信息,并生成为&str类型,方便后面解析用
pub fn get_code(url: &str) -> std::result::Result<String, Box<dyn std::error::Error>> {
    let code = reqwest::blocking::get(url)?.text()?;
    Ok(code)
}
// 以下是测试,因为代码都很简单,所有没有全部写测试,只测试了最拿不准的下载功能,可以运行cargo test看看效果
// 大多数都在main函数里面测试了,但当你的程序特别大的时候,单元测试是十分必要的
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

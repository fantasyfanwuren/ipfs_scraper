use std::sync::{Arc, Mutex};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 首先获取用户输入的网址以及线程数
    let (url, thread_num) = ipfs_scraper::get_input()?;
    let url = url.trim().to_string();

    // 根据网址获取网页源码
    let code = ipfs_scraper::get_code(&url)?;

    //解析源码
    let file_names = ipfs_scraper::analysis(&code);
    // println!("{:?}", file_names);

    // 创建文件夹(提取哈希值)
    let dir_vec: Vec<&str> = url.split("/").collect();
    let mut dir_path = dir_vec[0];
    for d in dir_vec {
        if d.len() > dir_path.len() {
            dir_path = d;
        }
    }
    let dir_path = ipfs_scraper::make_dir(&dir_path)?;
    println!("{:?}", dir_path);

    // 创建原子引用及互斥锁
    // index 用于流程控制,没开启一个线程,则index加1
    // file_names 是线程间需要不断访问的一个数组,里面存放着需要保存的文件名,通过与url进行字符串拼接可以获得要下载的地址
    // dir_path 也创建了锁,为了方便与file_names合成一个文件路径使用

    let index = Arc::new(Mutex::new(0));
    let file_names = Arc::new(Mutex::new(file_names));
    let url = Arc::new(Mutex::new(url));
    let dir_path = Arc::new(Mutex::new(dir_path));
    // 创建一个存放线程的数组,用于在线程全部启动后,等待他们全部结束后,退出程序
    let mut handles = vec![];
    for i in 0..thread_num {
        // 得到需要进入线程的数据
        let index = Arc::clone(&index);
        let url = Arc::clone(&url);
        let file_names = Arc::clone(&file_names);
        let dir_path = Arc::clone(&dir_path);
        let handle = std::thread::spawn(move || {
            'a: loop {
                //给数据上锁
                let mut num1 = index.lock().unwrap();
                let link = url.lock().unwrap();
                let file_names = file_names.lock().unwrap();
                let dir_path = dir_path.lock().unwrap();
                // 设置线程结束条件,当发现所有的filenames被遍历完了之后,就退出,这对于每个线程都是平等的
                if *num1 >= file_names.len() {
                    break 'a;
                }
                // 得到 下载链接 和 文件地址
                let division = if link.ends_with("/") { "" } else { "/" }; //如果链接后面有"/"就什么都不加,如果没有就加上
                let download_link = format!("{}{}{}", link, division, file_names[*num1]); // 通过字符串的拼接,拼接出当前文件的下载链接
                let download_file_path =
                    format!("{}/{}", dir_path.to_str().unwrap(), file_names[*num1]); // 通过字符串的拼接,拼接出当前文件的下载路径(含文件名)
                *num1 += 1;
                println!("线程{i}:{download_file_path}开始下载");
                // 给每个进入线程的数据进行解锁,让其他线程开始启动,注意,一个都别漏掉,漏掉一个都会成为死锁程序
                // 如果对多线程有任何疑问,可以参考学习一下文档后,再来看代码,绝对能看明白:
                // 闭包: https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html
                // 智能指针: https://kaisery.github.io/trpl-zh-cn/ch15-00-smart-pointers.html
                // 互斥锁:https://kaisery.github.io/trpl-zh-cn/ch16-03-shared-state.html
                drop(num1);
                drop(link);
                drop(file_names);
                drop(dir_path);
                // 这里之所以要加入循环是为了防止下载超时的错误导致线程退出,从而降低下载效率和准确性
                // 循环可以让下载超时后,继续重新下载.这对ipfs下载十分有用,就算网络很差也不用怕
                'b: loop {
                    match ipfs_scraper::download(&download_link, &download_file_path) {
                        Ok(_) => {
                            println!("线程{i}:{download_file_path}下载完成");
                            break 'b;
                        }
                        Err(_) => (),
                    }
                }
            }
        });
        handles.push(handle);
    }
    // 这里是为了让主线程等待所有线程结束后再退出程序用的一个等待
    // 可以试试删除这段代码会怎样.
    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

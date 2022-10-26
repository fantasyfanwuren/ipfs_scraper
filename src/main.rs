use std::sync::{Arc, Mutex};

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // 首先获取用户输入的网址以及线程数
    let (url, thread_num) = ipfs_scraper::get_input()?;
    let url = url.trim().to_string();

    // 根据网址获取网页源码
    let code = ipfs_scraper::get_code(&url)?;

    //解析源码
    let file_names = ipfs_scraper::analysis(&code);

    // 创建文件夹(文件名选取链接的10-20之间的字符,这是哈希值的一部分)
    let dir_path = ipfs_scraper::make_dir(&url[10..20])?;

    // 创建原子引用及互斥锁
    // index 用于流程控制,没开启一个线程,则index加1
    // file_names 是线程间需要不断访问的一个数组,里面存放着需要保存的文件名,通过与url进行字符串拼接可以获得要下载的地址

    let index = Arc::new(Mutex::new(0));
    let file_names = Arc::new(Mutex::new(file_names));
    let url = Arc::new(Mutex::new(url));
    let dir_path = Arc::new(Mutex::new(dir_path));
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
                // 设置线程结束条件
                if *num1 >= file_names.len() {
                    break 'a;
                }
                // 得到 下载链接 和 文件地址
                let download_link = format!("{}{}", link, file_names[*num1]);
                let download_file_path =
                    format!("{}/{}", dir_path.to_str().unwrap(), file_names[*num1]);
                *num1 += 1;
                println!("线程{i}:{download_file_path}开始下载");
                // 给每个进入线程的数据进行解锁,让其他线程开始启动
                drop(num1);
                drop(link);
                drop(file_names);
                drop(dir_path);
                //std::thread::sleep(std::time::Duration::from_millis(100));
                'b: loop {
                    match ipfs_scraper::download(&download_link, &download_file_path) {
                        Ok(_) => break 'b,
                        Err(_) => continue 'b,
                    }
                }
                println!("线程{i}:{download_file_path}下载完成");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    Ok(())
}

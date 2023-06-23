/// 读取txt文件中的数据
/// 此代码已失效
fn read_file(id: i32) -> String {
    let path = format!("src/data/{}.txt", id);

    //read_file
    let count = fs::read(path).unwrap();
    let buffer = std::str::from_utf8(&count[0..count.len()]).expect("msg");
    //println!("{}",buffer);
    buffer.to_string()
}
/// 向txt文件中写入数据
/// 此代码已失效
fn write_file(id: i32, count: String) -> Result<(), io::Error> {
    let path = format!("src/data/{}.txt", id);
    let mut file = fs::File::create(path).unwrap();
    let msg_buffer: i32 = string_to_i32(count);
    let msg = msg_buffer + 1;
    let msg_str = msg.to_string();
    let _ = file.write_all(msg_str.as_bytes());
    Ok(())
}
/// 创建txt文件
/// 此代码已失效
fn create_file(id: i32) -> Result<(), io::Error> {
    let path = format!("src/data/{}.txt", id);
    let mut file = fs::File::create(path).unwrap();
    let _ = file.write_all(b"0");
    Ok(())
}
/// 检查txt文件是否存在
/// 此代码已失效
fn check_file(id: i32) -> bool {
    let path = format!("src/data/{}.txt", id);
    let f = File::open(path);
    let result = match f {
        Ok(_file) => true,
        Err(_err) => false,
    };
    result
}

/// 将string类型的数据转换位i32
/// 此代码已失效
fn string_to_i32(string: String) -> i32 {
    string.parse().unwrap()
}

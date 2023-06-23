use serde_json::Value;

/// 从json文件中获取数据
/// 此代码正在使用中
pub fn get_info(mut data: Value, student: usize) -> u64 {
    //读取数据
    if let Some(people) = data.as_array_mut() {
        if let Some(person) = people.get_mut(student) {
            if let Some(count) = person["count"].as_u64() {
                count
            } else {
                todo!();
            }
        } else {
            todo!();
        }
    } else {
        todo!();
    }
}
/// 将usize类型的数据转换为i32
/// 此代码正在使用中
pub fn usize_to_i32(usize: usize) -> i32 {
    usize as i32
}

#![windows_subsystem = "windows"]
/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use rand::Rng;
use serde_json::Value;

#[derive(Default, NwgUi)]
pub struct BasicApp {
    //设置窗体大小和窗体标题
    #[nwg_control(size: (300, 300), position: (300, 300), title: "点名程序", flags: "WINDOW|VISIBLE")]
    //当程序关闭时显示弹窗
    #[nwg_events( OnWindowClose: [BasicApp::when_app_close] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,
    //flex:nwg::FlexboxLayout,
    //#[nwg_control(text: "Heisenberg", focus: true)]
    //#[nwg_layout_item(layout: grid, row: 0, col: 0,size:2)]
    //edit: nwg::TextBox,
    #[nwg_control(text: "抽取学生")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, row_span: 2)]
    #[nwg_events( OnButtonClick: [BasicApp::get_student] )]
    button: nwg::Button,
}

impl BasicApp {
    fn get_student(&self) {
        let mut rng = rand::thread_rng();
        let stduents = [
            "汪立思",
            "王金熙",
            "桂英涛",
            "丁李湘怡",
            "严欢怡",
            "彭文轩",
            "宁志现",
            "邓轲文",
            "陈文雪",
            "陈博",
            "程雯",
            "桂琦",
            "刘梦怡",
            "韩明轩",
            "李磊",
            "张正",
            "王馨悦",
            "晏永鸿",
            "葛照祥",
            "胡子恒",
            "洪凯乐",
            "蒋雨阳",
            "李研",
            "魏灿",
            "胡梦樊",
            "胡闻博",
            "李勋南",
            "左知恩",
            "张亚杰",
            "张广杰",
            "丁明锐",
            "余雪霏",
            "刘心语",
            "冯琳昱",
            "李诗妍",
            "郑雨萱",
            "王学强",
            "戴凌翔",
            "傅瑞琦",
            "董彦欣",
            "李爽",
            "丁善颖",
            "肖冰涛",
            "黄妍",
            "何微",
            "雷明扬",
            "刘承业",
            "裴作佳",
            "王传浩",
            "查彤彤",
            "周宪波",
            "丁妍",
            "夏厚禹",
            "彭杰瑞",
        ];
        let student = rng.gen_range(0..stduents.len());
        let id = usize_to_i32(student);

        //first version 用文本文件存贮数据
        // if check_file(id) {

        //     let count = read_file(id);
        //     let _ = write_file(id, count.to_string());
        //     nwg::modal_info_message(
        //         &self.window,
        //         "恭喜",
        //         &format!(
        //             "恭喜 {} !\n目前你已经被抽到 {} 次",
        //             stduents[student],
        //             string_to_i32(count) + 1
        //         ),
        //     );
        // } else {
        //     let _ = create_file(id);
        //     let count = read_file(id);
        //     let _ = write_file(id, count.to_string());
        //     nwg::modal_info_message(
        //         &self.window,
        //         "恭喜",
        //         &format!(
        //             "恭喜 {} !\n目前你已经被抽到 {} 次",
        //             stduents[student],
        //             string_to_i32(count) + 1
        //         ),
        //     );
        // }

        //second version 用json文件存储数据

        let mut file = File::open("data/data.json").expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read data from file");

        // 将JSON字符串解析为serde_json::Value
        let mut data: Value = serde_json::from_str(&contents).unwrap();

        // 修改JSON数组中某个对象的某个值
        if let Some(people) = data.as_array_mut() {
            if let Some(person) = people.get_mut(student) {
                if let Some(count) = person["count"].as_u64() {
                    person["count"] = Value::from(count + 1);
                }
            }
        }

        let count = get_info(data.to_owned(), student);

        // 将修改后的JSON对象序列化为字符串并写回文件
        let json_string = serde_json::to_string_pretty(&data).unwrap();
        let mut file = File::create("data/data.json").expect("Unable to create file");
        file.write_all(json_string.as_bytes())
            .expect("Unable to write data to file");
        nwg::modal_info_message(
            &self.window,
            "恭喜",
            &format!("恭喜 {} !\n目前你已经被抽到 {} 次", stduents[student],count),
        );
    }
    //当软件关闭的时候
    fn when_app_close(&self) {
        //nwg::modal_info_message(&self.window, "再见!", &format!("欢迎下次使用!"));
        //nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

fn read_file(id: i32) -> String {
    let path = format!("src/data/{}.txt", id);

    //write_file
    //let mut file = fs::File::create(path).unwrap();
    //let _ = file.write_all(b"Hello");

    //read_file
    let count = fs::read(path).unwrap();
    let buffer = std::str::from_utf8(&count[0..count.len()]).expect("msg");
    //println!("{}",buffer);
    buffer.to_string()
}

fn write_file(id: i32, count: String) -> Result<(), io::Error> {
    let path = format!("src/data/{}.txt", id);
    let mut file = fs::File::create(path).unwrap();
    let msg_buffer: i32 = string_to_i32(count);
    let msg = msg_buffer + 1;
    let msg_str = msg.to_string();
    let _ = file.write_all(msg_str.as_bytes());
    Ok(())
}

fn create_file(id: i32) -> Result<(), io::Error> {
    let path = format!("src/data/{}.txt", id);
    let mut file = fs::File::create(path).unwrap();
    let _ = file.write_all(b"0");
    Ok(())
}

fn check_file(id: i32) -> bool {
    let path = format!("src/data/{}.txt", id);
    let f = File::open(path);
    let result = match f {
        Ok(_file) => true,
        Err(_err) => false,
    };
    result
}

fn get_info(mut data:Value,student:usize) ->u64{
    //读取数据
    if let Some(people) = data.as_array_mut() {
        if let Some(person) = people.get_mut(student) {
            if let Some(count) = person["count"].as_u64() {
                count
            }
            else {
                todo!();
            }
        }else {
            todo!();
        }
    }
    else {
        todo!();
    }
}

fn usize_to_i32(usize: usize) -> i32 {
    usize as i32
}

fn string_to_i32(string: String) -> i32 {
    string.parse().unwrap()
}

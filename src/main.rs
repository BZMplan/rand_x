#![windows_subsystem = "windows"]
/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/
extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;
use nwd::NwgUi;
use nwg::NativeUi;
use rand::prelude::*;

use rand_x::student_list::studnet_list;
use rand_x::tool_function::{get_info, usize_to_i32};

use serde_json::Value;
use std::{
    fs::File,
    io::{Read, Write},
};

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
    /// 获取学生数据
    fn get_student(&self) {
        let mut rng = rand::thread_rng();
        let stduents = studnet_list();
        let student = rng.gen_range(0..stduents.len());
        let _id = usize_to_i32(student);

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
            &format!(
                "恭喜 {} !\n目前你已经被抽到 {} 次",
                stduents[student], count
            ),
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

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

#[derive(Default, NwgUi)]
pub struct BasicApp {
    //设置窗体大小和窗体标题
    #[nwg_control(size: (300, 300), position: (300, 300), title: "点名程序", flags: "WINDOW|VISIBLE")]
    //当程序关闭时显示弹窗
    //#[nwg_events( OnWindowClose: [BasicApp::when_app_close] )]
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
        let stduents = ["a", "b", "c"];
        let student = rng.gen_range(0..stduents.len());
        let id = usize_to_i32(student);

        let path = format!("src/data/{}.txt", id);
        if check_file(path) {
            let _ = create_file(id);
        } else {
            let count = read_file(id);
            let _ = write_file(id, count.to_string());
            nwg::modal_info_message(
                &self.window,
                "恭喜",
                &format!(
                    "恭喜 {} !\n目前你已经被抽到 {} 次",
                    stduents[student],
                    string_to_i32(count) + 1
                ),
            );
        }
    }
    //当软件关闭的时候
    //fn when_app_close(&self) {
    //    nwg::modal_info_message(&self.window, "再见!", &format!("欢迎下次使用!"));
    //      nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
    //    nwg::stop_thread_dispatch();
    //}
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}

use std::{
    fs::{self, File},
    io::{self, Write},
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

fn check_file(bible_path: String) -> bool {
    let f = File::open(bible_path);
    let result = match f {
        Ok(_file) => true,
        Err(_err) => false,
    };
    result
}

fn usize_to_i32(usize: usize) -> i32 {
    usize as i32
}

fn string_to_i32(string: String) -> i32 {
    string.parse().unwrap()
}

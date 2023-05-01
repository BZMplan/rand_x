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
    #[nwg_events( OnButtonClick: [BasicApp::rand_name] )]
    button: nwg::Button,
}

impl BasicApp {
    fn rand_name(&self) {
        let mut rng = rand::thread_rng();
        let stduents = [
            ];
        let student = rng.gen_range(0..stduents.len());
       
        nwg::modal_info_message(&self.window, "恭喜", &format!("恭喜 {} !", stduents[student]));
    }
    //当软件关闭的时候
    fn when_app_close(&self) {
        nwg::modal_info_message(&self.window, "再见!", &format!("欢迎下次使用!"));
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

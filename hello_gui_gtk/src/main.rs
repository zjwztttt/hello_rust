use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk::Button;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("你好 世界!");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("我的GTK GUI程序")
        .child(&button)
        .build();

    // 设置窗口默认大小
    window.set_default_size(1000, 660);
    // 设置窗口是否可以被调整大小
    window.set_resizable(true);
    // 设置按钮在窗口中的对齐方式
    button.set_halign(gtk::Align::Center);
    button.set_valign(gtk::Align::Center);
    // 设置要用标题栏和边框装饰的窗口
    window.set_decorated(true);
    // 设置默认为全屏显示
    window.set_fullscreened(false);
    // 设置窗口关闭按钮是否可见
    window.set_deletable(true);
    // 禁用关闭按钮的点击事件
    window.set_hide_on_close(false);
    window.set_maximized(false);
    // 禁用窗口双击事件
    window.set_modal(false);
    // 设置窗口的透明度
    window.set_opacity(1.0);
    // 激活窗口所有事件
    window.set_sensitive(true);
    

    // 启动一个无模式窗口
    window.show();
    // 启动一个模态窗口
    //window.present();
    
}
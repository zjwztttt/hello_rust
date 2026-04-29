fn main() {
    // 运行slintUI窗体
    //let _ = MainWindow::new().expect("REASON").run();
    MainWindow::new().unwrap().run().unwrap();
    //HelloWorld::new().unwrap().run().unwrap();
}
// slint宏，创建 UI
slint::slint!{
    import {GroupBox, LineEdit, Button} from "std-widgets.slint";
    //创建一个带标题的窗口
    export component MainWindow inherits Window {
        title: "Main Window";
        min-width: 1920px;
        min-height: 1080px;
        
        VerticalLayout { 
            alignment:start;
            padding-left: 25px;
            padding-right: 25px;
    
            Text { 
                font-size: 27px;
                font-weight: 700;
                color: #6776FF;  
             }
    
            GroupBox{
                title:"lineEdit";
                LineEdit {
                    placeholder-text: "enter text";
                }
            }
    
            Button {
                text: "Click Me";
                clicked => { self.text = "Clicked"; }
            }
    
        }
    }
    //创建一个没有标题的窗口
/*    export component HelloWorld {
        width: 600px;
        height: 600px;
        // 定义一个 Text 组件
        Text {
            text: "hello world";
            color: green;
        }
    }*/
}
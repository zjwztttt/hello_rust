use std::io;
use std::process::Command;

///定义名为Cat的结构体
struct Cat {
    cat_age: u8,
}

///实现块
impl Cat {
    ///结构体方法
    fn age_information(&self) ->u8 {
        self.cat_age
    }
}
///程序入口，main函数
fn main() {

    //从终端读取输入并转换数据为u8无符号整数类型
    println!("请输入猫的年龄");
    let mut age_input = String::new();
    io::stdin().read_line(&mut age_input).expect("未获取到有效的输入");
    let cat_age = age_input.trim().parse::<u8>();

    //处理终端的输入
    match cat_age {
        //正确输入处理
        Ok(cat_age) =>{
            //创建(申明)结构体实列
            let cat_strutc  = Cat{
                cat_age,
            };
            //输出结果
            println!("年龄是：{}", cat_strutc.age_information());
            //命令提示符 pause
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); 
        },
        //错误输入处理
        Err(_) => {
            //输出错误提示
            println!("请输入有效的值！");
            //命令提示符 pause
            let _ = Command::new("cmd.exe").arg("/c").arg("pause").status(); 
        }
    }
}
// 导入wasm_bindgen库
extern crate wasm_bindgen;
// 从wasm_bindgen库中导入prelude模块
use wasm_bindgen::prelude::*;

struct Number{
    a: isize,
    b: isize,
}

impl Number {
    fn print_sum(&self)->isize {
        self.a+self.b
    }
}

// 声明一个名为sum的导出函数
#[wasm_bindgen]
pub fn sum(a: isize, b: isize) -> isize {
    let num = Number{
        a,
        b,
    };
    return num.print_sum();
}

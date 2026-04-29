use std::io;
struct Cat{
   name: String,
   age: u128,
   color: String,
   species: String,
   price: f64,
   sale_status: bool,
}
fn main() {
   
   println!("请输入猫的名字");
   let mut name_input = String::new();
   io::stdin().read_line(&mut name_input).expect("Failed to read line");
   let cat_name = name_input.trim().to_string();
   
   println!("请输入猫的年龄");    
   let mut age_input = String::new();
   io::stdin().read_line(&mut age_input).expect("没有读取到有效的输入！");
   let cat_age = age_input.trim().parse::<u128>().expect("请输入有效的数字");
   
   println!("请输入猫的颜色");
   let mut color_input = String::new();
   io::stdin().read_line(&mut color_input).expect("获取不到有效的输入");
   let cat_color = color_input.trim().to_string();
   
   println!("请输入猫的品种");
   let mut species_input = String::new();
   io::stdin().read_line(&mut species_input).expect("没有读取到有效的输入");
   let cat_species = species_input.trim().to_string();
   
   println!("请输入商品零售价");
   let mut price_input = String::new();
   io::stdin().read_line(&mut price_input).expect("没有获取到有效的输入");
   let cat_price = price_input.trim().parse::<f64>().expect("请输入浮点数类型");
   
   println!("是否已售出？(true or false)");
   let mut sale_status_input = String::new();
   io::stdin().read_line(&mut sale_status_input).expect("没有获取到有效的输入");
   let cat_sale_status = sale_status_input.trim().parse::<bool>().expect("请输入true或false");
   
   #[warn(dead_code)]
   let cat_example = Cat{
      name: cat_name,
      age: cat_age,
      color: String::from(cat_color),
      species: String::from(cat_species),
      price: cat_price,
      sale_status: cat_sale_status,
   };
   println!("名字：{} 年龄：{} 颜色：{} 品种：{} 价格：{} 是否售出：{}",cat_example.name,cat_example.age,cat_example.color,cat_example.species,cat_example.price,cat_example.sale_status);
}
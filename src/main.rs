// 单行注释
/*多行注释
多行
*/

/// 文档注释
/// 比如你在src里有一个 hello.rs ，这时候你要在 main.rs 里写 （pub）mod hello；自动补齐才可以用。
pub mod a1_hello_rust;
pub mod a2_variate;
pub mod a3_data_type;
mod a4_tup;
mod a5_array;
mod a6_function;
mod a7_condition;
mod a8_while_for_loop;
mod a9_syq;
mod a3a_string;
mod a9a_yinyong;
mod a10_struct;


//main函数
fn main() {
    println!("你好 rust")
}
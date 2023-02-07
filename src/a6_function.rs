//

fn funct1(a: i16) -> f32 {
    println!("函数参数={}", a);
    return 3.14;
}

// 嵌套函数
fn fn1(a: i8) -> i32 {
    let b = a + 1;
    fn fn2(c: i32) -> i32 {
        println!("内层函数{}", c);
        //不带分号结尾，表示返回
        c + 3
    }
    //强制转换为 i32
    fn2(b as i32)
}


#[cfg(test)]
mod tests {
    use crate::a6_function::{fn1, funct1};

    #[test]
    fn t1() {
        let a = funct1(3);
        println!("{}", a)
    }

    #[test]
    fn t2() {
        let a = fn1(3);
        println!("a={:?}", a)
    }
}
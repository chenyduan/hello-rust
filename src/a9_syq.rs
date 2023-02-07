/// 所有权
// Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        let a = 1;

        println!("{}", a);
        {
            let b = a;
            println!("{}", b);
        }
        // 报错,当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)
        // println!("{}", b);

    }

    #[test]
    fn t2() {
        let a = "你好";
        let b =a;
        // 警告： a的所有权再上面转移给b后，a可以立刻被丢弃（drop）
        println!("所有权a={}", a);
        println!("所有权b={}", b);


    }
    #[test]
    fn t3(){
        let a = "你好";
        let b = a.clone();
        println!("所有权a={}", a);
        println!("所有权b={}", b);
    }

}
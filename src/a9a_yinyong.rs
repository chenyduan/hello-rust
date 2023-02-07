/// 引用
#[cfg(test)]
mod tests {
    #[test]
    fn t1() {
        let a = 30;
        // 引用
        let b = &a;
        println!("a={}", a);
        println!("*b={}", *b);
    }

    /// 可变引用
    #[test]
    fn t2() {
        let a = 5;
        let b = 6;
        let c = &a;
        println!("a={},{},{}", a, b, *c);
        // c = &b;  // 报错，默认引用也是不可变的
    }
}
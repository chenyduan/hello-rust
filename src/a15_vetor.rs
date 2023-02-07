/// 向量 vector  可以重复
#[cfg(test)]
mod test {
    // 创建向量
    #[test]
    fn t1() {
        // 通过数组创建
        let mut ve = vec![1, 2, 3];
        ve.push(4);
        println!("{:?}", ve);

        // 带泛型

        let mut b: Vec<i8> = Vec::new();
        b.push(1);
        b.push(1);
        b.push(2);
        b.push(3);
        println!("{:?}", b);
    }

    /// 遍历
    #[test]
    fn t2() {
        let mut a: Vec<i32> = Vec::new();
        a.push(1);
        a.push(3);
        a.push(5);
        a.push(7);
        for val in &a {
            println!("val = {} ", val)
        }

        // 遍历时改变值
        for i in &mut a {
            *i += 1;
        }
        println!("加1后的值{:?}", a)
    }
}

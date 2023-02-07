// 循环


#[cfg(test)]
mod tests {
    // for 循环
    #[test]
    fn t1() {
        let a = [3, 5, 7, 9];

        for ele in 0..4 {
            println!("遍历{}", a[ele])
        }


        for value in a {
            println!("值={}", value)
        }
    }

    /// while循环
    #[test]
    fn t2() {
        let b = [2, 4, 6, 8];
        let mut index = 0;
        while index < b.len() {
            println!("while={}", b[index])
            ;
            index += 1;
        }
    }

    /// loop 循环
    #[test]
    fn t3() {
        let a = [1, 2, 3, 4];
        let mut index = 0;
        // 类似于 while(true)
        loop {
            if a[index] > 3 {
                break;
            }
            println!("a={}", a[index]);
            index += 1;
        }
    }
}
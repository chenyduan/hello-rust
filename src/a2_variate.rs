


// 单元测试模块
#[cfg(test)]

mod tests{

    #[test]
    fn test1(){
        println!("单元测试")
    }
    //变量绑定，
    #[test]
    fn variate(){
        let a = "你好";
        println!("{}",a);
        let b = 1;
        let c = 3.14;
        println!("{}",b);
        println!("{}",c);

        // let 声明的不可变，此行会报错
        // a="变化"
    }



    // 可变
    #[test]
    fn test_mut(){
        let mut a = 5;
        println!("{}",a);
        a = 6;
        println!("{}",a);
    }


}

/// 引用其它文件

#[cfg(test)]
mod test{
    use crate::a12_generics::Animal;

    #[test]
    fn t1(){
        let a = Animal{
            a:"str"
        };
    }
}

///结构体

struct Animal {
    name: String,
    age: u8,
}


#[cfg(test)]
mod tests {
    use crate::a10_struct::Animal;

    #[test]
    fn t1() {
        // 创建实例
        // 初始化实例时，每个字段都需要进行初始化
        // 初始化时的字段顺序不需要和结构体定义时的顺序一致
        let dog = Animal {
            name: String::from("狗子"),
            age: 3,
        };
        // 报错
       //  println!("{:?}",dog);
        println!("{},{}",dog.name,dog.age);


        let dog2 = Animal{
            name :String::from("狗2"),
            age:dog.age
        };
        println!("dog2={},{}",dog2.name,dog2.age);


        //另一种写法
        let dog3 =Animal{
            name: String::from("狗3"),
            // 自动赋值其它字段
            ..dog
        };
        println!("dog3={},{}",dog3.name,dog3.age);

    }

    /// 可变引用
    #[test]
    fn t2() {}
}
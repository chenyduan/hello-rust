/// 枚举
enum Animal{
    DOG(i32),PIG(i32)
}

#[cfg(test)]
mod tests{
    use crate::a10a_enum::Animal;

    #[test]
    fn t1(){
        let animal = Animal::DOG(13);
    }

}
/// 方法


struct Animal {
    name: String,
    age: i8,
    weight: f32,
}

impl Animal {
    fn new(name: String, age: i8, weight: f32) -> Animal {
        Animal { name: name, age: age, weight }
    }

    /*
    self、&self 和 &mut self
我们使用 &self 替代 rectangle: &Rectangle，&self 其实是 self: &Self 的简写（注意大小写）。在一个 impl 块内，Self 指代被实现方法的结构体类型，self 指代此类型的实例，换句话说，self 指代的是 Rectangle 结构体实例，这样的写法会让我们的代码简洁很多，而且非常便于理解：我们为哪个结构体实现方法，那么 self 就是指代哪个结构体的实例。

，self 依然有所有权的概念：

self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
&self 表示该方法对 Rectangle 的不可变借用
&mut self 表示可变借用
    */


    fn brak(&self){
        println!("{}",self.name)
    }
}


#[cfg(test)]
mod test{
    use crate::a11_method::Animal;

    #[test]
    fn t1(){
      let dog =  Animal::new(String::from("狗"),12,10.0);
        dog.brak()
    }
}
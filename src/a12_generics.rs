/// 泛型


pub(crate) struct Animal<T>{
    pub(crate) a:T
}


enum Tree<E>{
    a(E)
}


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



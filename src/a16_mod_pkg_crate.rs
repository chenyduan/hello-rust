
/// mod  module 模块   mod A 和 B
/// 整个文件称为 crate

pub(crate) mod Ab{
    pub(crate) mod A{
        pub(crate) fn a1(){
            println!("模块A");
        }


        struct As{
            name:String,
            age:i32
        }

    }

    mod B{
        fn b1(){
            println!("模块B")
        }
    }
}


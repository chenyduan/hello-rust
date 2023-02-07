/// package   cargo new my-project
/// cargo new my-lib --lib
///
///
///
///
mod C {
    fn c() {}
}

pub fn cr() {
    println!("调用其他模块");
    crate::a16_mod_pkg_crate::Ab::A::a1();
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use crate::a16a_mod2mod::cr;

    #[test]
    fn t1() {
        cr();
    }
}

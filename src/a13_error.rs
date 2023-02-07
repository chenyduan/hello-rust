/// 异常
#[cfg(test)]
mod test {
    use std::fs::File;
    #[test]
    fn t1() {
        let f = File::open("a.txt");
        match f {
            Ok(str) => {
                println!("ok")
            }
            Err(err) => {
                println!("err")
            }
        }
    }
}

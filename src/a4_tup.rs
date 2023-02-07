#[cfg(test)]
mod tests {
    #[test]
    fn test_tup() {

        // 元组 ,可以存放不同类型
        let a = (0, 3.14, true);

        println!("a={:?}", a);
        let (a1, a2, a3) = a;
        println!("a1={},a2={},a3={}", a1, a2, a3)
    }
}
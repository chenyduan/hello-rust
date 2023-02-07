#[cfg(test)]
mod tests {
    #[test]
    fn test_array() {
        let a:[i32;4] = [4, 1, 2, 0];
        println!("{}", a[1])
    }
}
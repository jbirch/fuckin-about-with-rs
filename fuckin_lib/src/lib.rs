#[cfg(test)]
mod tests {
    use my_add;

    #[test]
    fn it_works() {
        assert_eq!(my_add(2, 2), 4);
    }
}

pub fn my_add(a: i32, b: i32) -> i32 {
    a + b
}

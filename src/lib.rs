pub mod timeutils;
#[cfg(test)]
mod tests {
    use crate::timeutils::get_current_unix_timestamp;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_current_unix_timestamp() {
        println!("hello world");
        assert_eq!(0, get_current_unix_timestamp());
    }
}

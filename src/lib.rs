pub mod timeutils;
#[cfg(test)]
mod tests {
    use crate::timeutils::{
        get_current_unix_timestamp_as_milsecs, get_current_unix_timestamp_as_secs,
    };

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_get_current_unix_timestamp_as_secs() {
        println!("hello world");
        let ts_now = get_current_unix_timestamp_as_secs();
        println!("tsNow: {}", ts_now);
        assert_ne!(0, ts_now);
    }
    #[test]
    fn test_get_current_unix_timestamp_as_milsecs() {
        println!("hello world");
        let ts_now = get_current_unix_timestamp_as_milsecs();
        println!("tsNow: {}", ts_now);
        assert_ne!(0, ts_now);
    }
}

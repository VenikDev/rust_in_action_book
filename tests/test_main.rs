#[cfg(features = "unit")]
mod tests {
    use super::*;

    #[test]
    fn add(){
        assert_eq!(1 + 4, 5);
        assert_eq!(4 + 4, 8);
        assert_eq!(7 + 4, 11);
    }
}
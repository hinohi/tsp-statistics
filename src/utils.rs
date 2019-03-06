/// 2数を(大きくない方、もう片方)の順に並び替える
pub fn order_ab<T: Ord>(a: T, b: T) -> (T, T) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_ab() {
        assert_eq!(order_ab(1, 2), (1, 2));
        assert_eq!(order_ab(2, 1), (1, 2));
        assert_eq!(order_ab(2, 2), (2, 2));
    }
}

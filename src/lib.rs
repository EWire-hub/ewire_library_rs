/// 競プロ用ライブラリ
///

pub use tmp;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        let result2 = tmp::sub(2, 2);
        assert_eq!(result2, 0);
    }
}

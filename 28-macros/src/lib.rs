#[macro_export]
macro_rules! sum {
    ( $( $x:expr ),* ) => {
        {
            let num = 0;
            $(
                let num = num + $x;
            )*
            num
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

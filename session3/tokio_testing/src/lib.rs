async fn double(num: u32) -> u32 {
    num * 2
}

#[cfg(test)]
mod test {

    use super::*;

    /*#[test]
    fn test_double_will_not_compile() {

        assert_eq!(double(3).await,6); // needs an async runtime
    }*/

    #[test]
    fn test_double_verbose() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        let future_res = rt.block_on(double(3));
        assert_eq!(future_res, 6);
    }


    #[tokio::test]
    async fn test_double(){
        assert_eq!(double(3).await,6);
    }
}

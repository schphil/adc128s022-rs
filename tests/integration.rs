#[cfg(feature = "blocking")]
mod test {
    #[test]
    fn can_create_and_destroy_adc128s022() {
        // let dev = new_ad9833(&[]);
        // destroy(dev);
    }
}

#[cfg(feature = "async")]
mod test {
    fn can_create_and_destroy_adc128s022() {
        // let dev = new_ad9833(&[]);
        // destroy(dev);
    }
}

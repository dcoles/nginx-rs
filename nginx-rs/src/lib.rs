pub mod http;
pub mod bindings;
pub mod core;
pub mod log;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

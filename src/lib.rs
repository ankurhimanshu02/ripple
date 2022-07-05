pub mod commands;
pub mod commitment;
pub mod config;
pub mod configuration;
pub mod errors;
pub mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

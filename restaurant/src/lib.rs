mod front_of_house;
mod great;

pub use crate::front_of_house::hosting;
pub use crate::great::amber;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_of_house {
    fn hello () {}

    pub mod hosting {
        pub fn add_to_waitlist() {
            // 放稳同一个包下的方法
            seat_at_table();
            super::serving::take_order();
            super::hello();
        }

        fn seat_at_table() {}
    }

    mod serving {
        pub fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}


pub fn eat_at_restaurant () {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_front_of_house() {
        crate::front_of_house::hosting::add_to_waitlist()
    }
}

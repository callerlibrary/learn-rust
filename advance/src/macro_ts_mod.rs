pub mod macro_ts {
    pub fn ts() {
        println!("hello macro");
    }

    #[macro_export]
    macro_rules! vec_ts {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
}

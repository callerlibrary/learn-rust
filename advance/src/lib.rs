// 生命周期
pub mod lifetime {
    pub fn life_cycle() {
        struct Interface<'b, 'a: 'b> {
            manager: &'b mut Manager<'a>,
        }

        impl<'b, 'a: 'b> Interface<'b, 'a> {
            pub fn noop(self) {
                println!("interface consumed");
            }
        }

        struct Manager<'a> {
            text: &'a str,
        }

        struct List<'a> {
            manager: Manager<'a>,
        }

        impl<'a> List<'a> {
            pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
            where
                'a: 'b,
            {
                Interface {
                    manager: &mut self.manager,
                }
            }
        }

        fn use_list(list: &List) {
            println!("{}", list.manager.text);
        }

        let mut list: List<'_> = List {
            manager: Manager { text: "hello" },
        };

        list.get_interface().noop();

        println!("Interface should be dropped here and the borrow released");

        // 下面的调用会失败，因为同时有不可变/可变借用
        // 但是Interface在之前调用完成后就应该被释放了
        use_list(&list);
    }
}

// 闭包
pub mod closure {
    pub fn closure() {
        let mut x: i32 = 999i32;
        let mut c = || x += 1;
        c();
        c();
        c();
        println!("{}", x);
    }
}

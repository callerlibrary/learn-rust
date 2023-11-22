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
    use std::thread;
    use std::time::Duration;
    pub fn closure() {
        let mut x: i32 = 999i32;
        let mut c = || x += 1;
        c();
        c();
        c();
        println!("{}", x);
    }

    // 开始健身，好累，我得发出声音：muuuu...
    fn muuuuu(intensity: u32) -> u32 {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    fn workout(intensity: u32, random_number: u32) {
        if intensity < 25 {
            println!("今天活力满满，先做 {} 个俯卧撑!", muuuuu(intensity));
            println!(
                "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
                muuuuu(intensity)
            );
        } else if random_number == 3 {
            println!("昨天练过度了，今天还是休息下吧！");
        } else {
            println!(
                "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
                muuuuu(intensity)
            );
        }
    }

    pub fn run() {
        // 强度
        let intensity = 10;
        // 随机值用来决定某个选择
        let random_number = 7;

        // 开始健身
        workout(intensity, random_number);
    }
}

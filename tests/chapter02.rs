
#[cfg(test)]
#[allow(unused_imports)]
pub mod chapter02 {
    use std::thread;

    #[test]
    fn test_01() {
        pub struct Duck;
        pub struct Pig;
        pub trait Fly {
            fn fly(&self) -> bool;
        }

        impl Fly for Duck{
            fn fly(&self) -> bool {
                true
            }
        }

        impl Fly for Pig{
            fn fly(&self) -> bool {
                false
            }
        }

        fn fly_static<T: Fly>(s: T)-> bool {
            s.fly()
        }

        fn fly_dyn(s: &dyn Fly) -> bool {
            s.fly()
        }

        assert_eq!(fly_dyn(&Pig),false);
        assert_eq!(fly_dyn(&Duck),true);
    }

    // 定义全局静态变量，并在unsafe block 中修改，操作它
    #[test]
    fn test_02(){
        static mut A :i32 = 10;
        unsafe {
            A = 20;
        }
        unsafe {
            println!("{:?}",A)
        }
    }

    // 具名结构体定义
    #[test]
    fn test_03(){
        #[derive(Debug,PartialEq)]
        struct People{
            name: &'static str,
            gender: u32,
        }

        impl People{
            // People 的关联函数
            fn new(name: &'static str,gender: u32)->Self{
                People{
                    name,
                    gender
                }
            }
            pub fn name(&self){
                println!("{:?}",self.name)
            }

            pub fn set_name(&mut self,name: &'static str){
                self.name = name
            }

            pub fn gender(&self){
                let gender = if self.gender == 1 {"boy"}else{"girl"};
                println!("{:?}",gender)
            }
        }

        let alex = People::new("Alex",1);
        alex.name();
        alex.gender();
        assert_eq!(alex,People{name:"Alex",gender:1});
    }

    #[test]
    fn test_04(){
        pub fn temp() ->i32 {
            1
        }
        let x = &temp();
        println!("{:?}",x);

        let a = 1;
        let mut b = 2;
        b = 3;

        let mut v:Vec<i32> = vec![1,2,3];
        let h = thread::spawn(move||{
            v.push(4);
            println!("{:?}",v);
        });

        fn inner_func(vref: &mut Vec<u32>){
            thread::spawn(move||{
                vref.push(3);
            });
        }
        let mut v = vec![1,2,3];
        inner_func(&mut v);
    }

    pub type Array = Vec<People>;
    pub type Hash = LinkedList<Yaml>;
    // self 代指当前 mod
    #[test]
    fn test_05(){
        pub enum People {
            Man(self::Hash), // self 代指当前 mod
            Woman(self::Array),
        }
    }
}

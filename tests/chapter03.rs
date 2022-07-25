
#[cfg(test)]
mod test{
    use std::borrow::{Borrow, BorrowMut};
    use std::cell::{Cell, Ref, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::time::Duration;
    use salvo::http::headers::Te;
    use tracing_subscriber::fmt::try_init;

    #[derive(Debug,Clone)]
    struct User {
        name: String,
        age: RefCell<u32>,
        gender: bool,
        height: Cell<u32>,
        email: RefCell<String>,
    }
    impl Drop for User{
        fn drop(&mut self) {
            println!("user is drop!")
        }
    }

    #[derive(Debug)]
    pub struct Teacher {
        name: String,
        curriculums: Vec<String>,
        num: i32,
    }
    
    impl Drop for Teacher{
        fn drop(&mut self) {
            println!("teacher is drop!")
        }
    }

    // Box智能指针
    // Copy trait 是值语义的复制，rust编译器自动为基本数据类型实现该 trait
    // Clone trait 是引用语义的深复制，引用类型调用 clone 方法，以实现深度克隆。
    // 能 Copy 的就是值语义，不能的就是引用语义。
    #[test]
    fn test_box(){
        // 这是错误的写法，Box智能指针不能实现 Copy trait
        // #[derive(Debug,Copy,Clone)]
        // struct A {
        //     a: i32,
        //     b: Box<i32>, //Error，Box智能指针不能实现 Copy trait
        // }
        // let a = A{a:5,b:Box::new(6)};
        // println!("{:?}",a);

        // Box 唯一所有权智能指针能实现 Clone trait
        // #[derive(Debug,Clone)]
        // struct B {
        //     a: i32,
        //     b: Box<i32>,
        // }
        // let b = B{a: 3,b: Box::new(2)};
        // println!("{:?}",b);

        let mut map = HashMap::<String,i32>::new();
        map.insert("aa".to_string(),11);
        map.insert("bb".to_string(),22);
        assert_eq!(map.len(),2);
    }

    // 同一个HashMap存储不同的 Key Value 键值对
    #[test]
    fn test_hash_map(){
        // 这种实现方式报错了
        // trait IHashMap{}
        // struct Go {}
        // impl IHashMap for Go{}
        //
        // trait IHashMap2{}
        // struct Rust{}
        // impl IHashMap2 for Rust{}
        //
        // let mut hash_map = HashMap::<Box<dyn IHashMap>,Box<dyn IHashMap2>>::new();
        // hash_map.insert(Box::new(Go{}),Box::new(Rust{}));
        // assert_eq!(hash_map.len(),1);

        // let mut hash_map = HashMap::<T,U>::new();
        // hash_map.insert("aa",11);
        // hash_map.insert(22,"bb");
        // assert_eq!(2,hash_map.len());//Error
    }

    // 移动语义
    #[test]
    fn test_move_trait(){
        // 结构体未实现 Copy trait
        // #[derive(Debug)]
        // struct A {
        //     a: i32,
        //     b: u32,
        // }
        // let a = A{a: 2,b: 3};
        // let b = a; // 因为 A 结构体未实现 Copy trait, 此处 a的所有权已经被转移到了b
        // println!("{:?}",a);//Error 不能操作一个已经被 move 的 a

        // 这里 Copy/Clone 要都实现才不会报错，只实现 Copy 或者 Clone 就会报错
        #[derive(Debug,Copy,Clone)]
        struct B {
            a: i32,
            b: u32,
        }
        let bb = B{a: 5,b:6};
        let cc = bb;
        println!("{:?}",bb);
    }

    // 词法作用域，生命周期
    #[test]
    fn test_lifetime_scope(){
        let outer_val = 1;
        let outer_sp = "hello".to_string();
        {
            let inner_val = 2;
            outer_val; //outer_val 是复制语义，它的所有权未被转移
            outer_sp; //outer_sp 是移动语义，它的所有权就被转移到这个作用域中
        }
        println!("{:?}",outer_val);
        //println!("{:?}",inner_val);//Error, inner_val 不在当前作用域内 cannot find value `inner_val` in this scope
        //println!("{:?}",outer_sp); //Error, borrow of moved value: `outer_sp`
    }

    // match 匹配词法作用域
    #[test]
    fn test_match_lifetime_scope(){
        // match 匹配中的词法作用域
        // let a = Some("hello".to_string());
        // match a {
        //     Some(s) => println!("{:?}",s),
        //     _ => println!("nothing"),
        // }
        // println!("a = {:?}",a);//Error
    }

    // Vector 词法作用域
    #[test]
    fn test_vec_lifetime_scope(){
        let v = vec![1,2,3];
        for i in v {
            println!("{:?}",i);
            //println!("{:?}",v); Error value borrowed here after move
        }
        let vec = vec![4,5,6];
        for i in vec.iter().enumerate(){
            println!("index = {:?}",i.0);
            println!("value = {:?}",i.1);
        }
    }

    // if let 判断的词法作用域
    #[test]
    fn test_if_let_lifetime(){
        let a = Some("hello world".to_string());
        if let Some(s) = a {// 此时 a 的所有权被转移到这个表达式这里
            println!("{:?}",s);
            //println!("{:?}",a);//Error value borrowed here after partial move
        }
    }

    // 函数的词法作用域
    // 函数参数 s String 类型，是 move 语义，因此 s 的所有权被转移到了函数内部
    fn foo(s: String) -> String {
        let w = "world".to_string();
        s + &w
    }
    #[test]
    fn test_fn_lifetime_scope(){
        let s = "hello".to_string();
        let ss = foo(s);
        println!("{:?}",ss);
        //println!("{:?}",s); //Error value borrowed here after move
    }

    #[test]
    fn test_closure(){
        let mut optional = Some(0); // optional 为 Option<i32> 类型，为复制语义，
        while let Some(i) = optional{ // 判断 optional 的值，while let 后，变量i的所有权未转移
            if i > 9 {
                println!("Greater than 9,quit");
                optional = None;
            }else {
                println!("`i` is `{:?}`.Try again.",i);
                optional = Some(i+1);
            }
        }
    }

    #[test]
    fn test_closure2(){
        // let mut optional = Some("hello".to_string());
        // if let Some(i) = optional{// optional的所有权转移到了 if let 表达式中
        //     if i == "hell".to_string(){
        //         println!("{:?}",i);
        //     }else {
        //         println!("{:?}",Some(i+" "+&"world".to_string()))
        //     }
        // }

       let mut optional = Some(0);
        if let Some(i) = optional {
            if i > 9 {
                println!("Greater than 9,quit!",);
                optional = None;
            }else {
                println!("`i` is `{:?}`.Try again.",i);
                optional = Some(i+1);
            }
        }
        println!("i = {:?}",optional.unwrap());
    }

    // Cell RefCell 共享引用 (不可变变量，引用) 的内部可变性
    #[test]
    fn test_RefCell(){
        let user = User{ //
            name: "小明".to_string(),
            age: RefCell::new(19),
            gender: true,
            height: Cell::new(178),
            email: RefCell::new("123456@qq.com".to_string())
        };
        println!("{:#?}",user);
        println!("===========================================");
        let cell_address = user.height.set(199);
        let refcell_email = user.email.replace("32127286".to_string());
        println!("{:#?}",user);
    }
    
    fn save(user: Rc<User>,teacher: Rc<Teacher>){
        add(user,teacher)
    }
    
    fn add(user: Rc<User>,teacher: Rc<Teacher>){
        println!("{:#?}",user);
        println!("{:#?}",teacher);
    }

    // Rc 引用计数智能指针 在嵌套函数中的应用 (Rc 只能用在单线程中,默认只能修饰不可变引用、对象)
    #[test]
    fn test_embed_func(){
        let user = Rc::new(User{
            name: "韩信".to_string(),
            age: RefCell::new(23),
            gender: true,
            height: Cell::new(188),
            email: RefCell::new("1663309@163.com".to_string())
        });
        let teacher = Rc::new(Teacher{
            name: "赵老师".to_string(),
            curriculums: vec!["语文".to_string(),"数学".to_string()],
            num: 1
        });
        save(user.clone(),teacher.clone());
        println!("============================================================");
        println!("user = {:#?}",user);
        println!("teacher = {:#?}",teacher);
    }

    // 这个结构体的字段类型有 String 类型，并且String类型只实现了 Clone Trait,因此该 结构体也只能实现 Clone Trait,不能实现Copy Trait
    #[derive(Debug,Clone)]
    pub struct ApplicationConfig{
        pub debug: bool,
        pub server_url: String,
        pub redis_url: String,
        pub database_url: String,
        pub logic_column: String,
        pub logic_un_deleted: i64,
        pub logic_deleted: i64,
        pub log_dir: String,
        pub log_cup: i64,
        pub log_temp_size: String,
        pub log_zip: bool,
        pub log_rolling_type: String,
        pub log_level: String,
        pub sms_redis_send_key_prefix: String,
        pub jwt_secret: String,
        pub white_list_api: Vec<String>,
        pub cache_type: String,
        // 登陆失败重试
        pub login_fail_replay: i64,
        pub login_fail_retry_wait_sec: i64,
    }

    impl Default for ApplicationConfig {
        fn default() -> Self {
            Self{
                debug: false,
                server_url: "".to_string(),
                redis_url: "".to_string(),
                database_url: "".to_string(),
                logic_column: "".to_string(),
                logic_un_deleted: 0,
                logic_deleted: 0,
                log_dir: "".to_string(),
                log_cup: 0,
                log_temp_size: "".to_string(),
                log_zip: false,
                log_rolling_type: "".to_string(),
                log_level: "".to_string(),
                sms_redis_send_key_prefix: "".to_string(),
                jwt_secret: "".to_string(),
                white_list_api: vec![],
                cache_type: "".to_string(),
                login_fail_replay: 0,
                login_fail_retry_wait_sec: 0
            }
        }
    }
    
    #[test]
    fn test_impl_clone(){
        let cfg = ApplicationConfig::default();
        let config = cfg.clone();
        let config2 = cfg.clone();
        println!("cfg = {:?}",cfg);
        println!("cfg = {:?}",config);
        println!("cfg = {:?}",config2);
    }

    #[test]
    fn get_max(){
        // 定义一个 Vec<i32>
        let number_list = vec![34,50,25,100,65];
        // 定义一个最大值，默认是 number_list 的第一个元素
        let mut largest = number_list[0]; // 这里是正确的，但是 Clion 却提示错误
        // 遍历 number_list 中的元素，并将它的每一个元素都与默认的最大值比较
        for number in number_list {
            if number > largest {
                largest = number;
            }
        }
        println!("The largest number is {}",largest);
        let ti = Duration::from_secs(111);
    }

    #[derive(Debug,Clone)]
    pub struct Student<'a>{
        pub age: i32,
        pub name: String,
        pub email: &'a str,
        pub address: &'a str,
    }

    /// 获取 类型为 Move 语义的 Vec 中的元素
    #[test]
    fn test_vec_move(){

        // 类型为 Copy 语义
        let mut i32_list: Vec<i32> = Vec::new();
        i32_list.push(1);
        let res = i32_list[0];
        assert_eq!(1,res);

        // 类型为 Move 语义
        let  mut object_list: Vec<Student> = Vec::new();
        object_list.push(Student{
            age: 11,
            name: "小明".to_string(),
            email: "123456@qq.com",
            address: "南山"
        });

        // 错误的写法
        // let res = object_list[0]; //Error: Cannot move
        //
        // // 错误的写法
        // let res = object_list[0.clone()]; //Error: Cannot move

        // 正确的写法
        let res = object_list[0].clone();
        println!("{:?}",res);

        // 正确的写法
        let res = object_list.get(0);
        println!("{:?}",res);
    }

    // #[test]
    // fn test_rc_vec(){
    //     let mut object_list = Vec::from(Rc::new(Student));
    //     let st1 = Rc::new(Student{
    //         age: 0,
    //         name: "".to_string(),
    //         email: "",
    //         address: ""
    //     });
    //     object_list.push(st1);
    //     let res = object_list[0].clone();
    //     println!("{:?}",res);
    // }

    // 使用泛型 Fn trait 存储 closure (闭包)
    #[test]
    fn test_fn_trait_closure(){
        // 定义一个存储 Fn(i32) -> i32 闭包的结构体
        // struct Container<T> where T: Fn(i32) -> i32{
        //     calculation: T,
        //     value: Option<i32>,
        // }
        //
        // // impl 后面定义 T ，结构体 Container 后面使用 T
        // impl <T>Container<T> where T: Fn(i32) -> i32{
        //     fn new(calculation: T) -> Container<T>{
        //         Container{
        //             calculation,
        //             value: None,
        //         }
        //     }
        //
        //     //
        //     fn value(&mut self,arg: i32) -> i32{
        //         match self.value{
        //             Some(v) =>v, // 有值返回值
        //             None => { // 无值创造值
        //                 let v = self.calculation(arg);
        //                 self.value = Some(v);
        //                 v
        //             }
        //         }
        //     }
        // }


    }
}

#[cfg(test)]
mod test{
    use std::cell::{Cell, Ref, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;
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

    // Rc 单线程引用计数器
    #[test]
    fn test_Rc(){
        let user = User{name:"小明".to_string(),age: RefCell::new(18),gender: true, height: Cell::new(0), email: RefCell::new("".to_string()) };
        let rc_user = Rc::new(user);
        // clone() 指向 User 对象的引用 +1, (仅仅是引用 + 1 ，堆中的对象并没有被复制)
        let rc_user_01 = rc_user.clone();
        let rc_user_02 = rc_user.clone();
        let rc_user_03 = rc_user.clone();
        println!("{:?}",rc_user);
        println!("{:?}",rc_user_01);
        println!("{:?}",rc_user_02);
        println!("{:?}",rc_user_03);
        println!("调用完毕");
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
}


#[cfg(test)]
mod smart_pointer_test{
    use std::{thread, time};
    use std::cell::{Cell, RefCell};
    use std::hash::Hash;
    use std::ops::Index;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex, MutexGuard};

    #[derive(Debug)]
    struct People {
        name: String,
        age: RefCell<u32>, // 运行时进行借用检查
        gender: bool,
        height: Cell<u32>, // 编译时借用检查
        email: RefCell<String>,
    }

    impl Drop for People {
        fn drop(&mut self) {
            println!("People is drop !!!")
        }
    }

    #[derive(Debug)]
    struct Hello{
        v: i32
    }

    impl Hello{
        pub fn say_hello(&self){
            let id = thread::current().id();
            println!("{:?} {:p} ==> {}",id,self,self.v);
        }
    }
    
    impl Drop for Hello{
        fn drop(&mut self) {
            println!("hello is drop!");
        }
    }
    #[test]
    fn test_arc(){
        // 克隆的 h 指向的是同一块堆内存
        let h = Arc::new(Hello{v: 10});
        for _ in 0..10 {
            let h = h.clone(); // 克隆一个 h 对象的栈上指针，同时 h 指向的堆上引用计数器 +1 
            thread::spawn(move||{
                h.say_hello();
            });
        };
        h.say_hello(); //输出 ThreadId(2) 0x7fb9a8406a10 ==> 10
        h.say_hello(); //输出 ThreadId(2) 0x7fb9a8406a10 ==> 10
        h.say_hello(); //输出 ThreadId(2) 0x7fb9a8406a10 ==> 10
        thread::sleep(time::Duration::from_secs(1));
        /*
        ThreadId(4) 0x7fb9a8406a10 ==> 10
        ThreadId(3) 0x7fb9a8406a10 ==> 10
        ThreadId(5) 0x7fb9a8406a10 ==> 10
        ThreadId(8) 0x7fb9a8406a10 ==> 10
        ThreadId(7) 0x7fb9a8406a10 ==> 10
        ThreadId(6) 0x7fb9a8406a10 ==> 10
        ThreadId(11) 0x7fb9a8406a10 ==> 10
        ThreadId(2) 0x7fb9a8406a10 ==> 10
        ThreadId(2) 0x7fb9a8406a10 ==> 10
        ThreadId(2) 0x7fb9a8406a10 ==> 10
        ThreadId(9) 0x7fb9a8406a10 ==> 10
        ThreadId(10) 0x7fb9a8406a10 ==> 10
        ThreadId(12) 0x7fb9a8406a10 ==> 10
        hello is drop!
         */
    }

    #[derive(Debug)]
    struct World{
        v: i32
    }

    impl World{
        // &self 调用的只读方法
        pub fn say_hello(&self){
            let id = thread::current().id();
            println!("{:?} {:p} ==> {}",id,self,self.v);
        }
        // &mut self 调用的可读可写方法
        pub fn change(&mut self){
            (*self).v = 20; // self 是个可变引用，(*self) 就是取 self 这个引用指向的值
            self.v += 10;
        }
    }

    impl Drop for World{
        fn drop(&mut self) {
            println!("world is drop!");
        }
    }

    // 同一堆内存的多个所有权，并进行读写操作
    #[test]
    fn test_arc_mutex(){
        let w = Arc::new(Mutex::new(World{v: 10}));
        for _ in 0..10 {
            let w = w.clone(); //克隆一个 h 对象的栈上指针，同时该指针指向的堆上内存引用计数自动 +1
            thread::spawn(move||{
                let mut l: MutexGuard<World> = w.lock().unwrap(); //先获取 h 的锁
                l.change(); // 因为 MutexGuard 实现了 DerefMut ，l 进行可变解引用后可以执行写操作
                l.say_hello();// 同时 MutexGuard 也实现了 Deref ，l 进行不可变解引用后可以执行读操作
            }); // MutexGuard 的 drop 方法会自动调用 unlock() 方法(自动释放锁)。
        };
        thread::sleep(time::Duration::from_secs(1));
    }

    pub fn change(a: Arc<i32>){
        for _ in 0..10 {
            let a = a.clone();
            thread::spawn(move ||{
                println!("{:?}",a);
            });
        };
        println!("线程之外的 a {:?}",a);
    }

    pub fn embed(b: i32,a: i32){
        let a = a+ b;
        change(Arc::new(a))
    }

    // 在函数之中安全的传递 所有权
    #[test]
    fn test_arc_embed_func(){
        embed(1,2);
    }

    // 像引用一样使用智能指针
    #[test]
    fn test_box(){
        let x = 5;
        let y = Box::new(5);
        assert_eq!(5,x);
        //assert_eq!(5,y); //Error : 类型不一致，5是 i32 类型，而 y 是 &i32类型
        assert_eq!(5,*y);  //Ok: 因为 y 是引用类型，所有对 y 执行 * (解引用) 操作是可以的

        let s = "hello world";
        let ss = String::from("hello world"); // String 实现了 ops::Deref
        assert_eq!("hello world",s);
        assert_eq!("hello world",ss);
        println!("{}",&ss);

    }

    // Rc 单线程引用计数器 (生成环境中应该根据使用场景使用 Rc 或者 Arc ,而不是使用类型自带的 Clone trait
    #[test]
    fn test_Rc(){
        let people = People{name:"小明".to_string(),age: RefCell::new(18),gender: true, height: Cell::new(0), email: RefCell::new("".to_string()) };
        let rc_people = Rc::new(people);

        // 指向 User 对象的引用 +1, (仅仅是引用 + 1 ，堆中的对象并没有被复制)
        let rc_people_clone_01 = rc_people.clone();
        let rc_people_clone_02 = rc_people.clone();
        let rc_people_clone_03 = rc_people.clone();
        println!("{:?}",rc_people);
        println!("{:?}",rc_people_clone_01);
        println!("{:?}",rc_people_clone_02);
        println!("{:?}",rc_people_clone_03);

        println!("rc_user 对象的引用计数 = {:?}",Rc::strong_count(&rc_people));

        let rc_user_01 = Rc::clone(&rc_people);
        let rc_user_02 = Rc::clone(&rc_people);
        let rc_user_03 = Rc::clone(&rc_people);
        println!("rc_user 对象的引用计数 = {:?}",Rc::strong_count(&rc_people));
        println!("调用完毕");
    }
}
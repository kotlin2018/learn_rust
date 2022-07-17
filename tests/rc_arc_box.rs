

#[cfg(test)]
mod smart_pointer_test{
    use std::{thread, time};
    use std::sync::{Arc, Mutex, MutexGuard};

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
}
/// 线程
///
///
use std::thread;
use std::time::Duration;

fn spwn_fun() {
    for i in 0..5 {
        println!("i={}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;
    use crate::a14_thread::spwn_fun;

    #[test]
    fn t1() {
        // 主线程结束，子线程也会结束，并不会执行完
        thread::spawn(spwn_fun);
        for i in 0..3 {
            println!("main={}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }


    #[test]
    fn t12() {
        let a = thread::spawn(|| {
            for i in 0..5 {
                println!("子线程={}", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 0..3 {
            println!("main={}", i);
            thread::sleep(Duration::from_millis(1));
        }
        // 主线程结束，子线程不会结束，
        a.join().unwrap();
    }


    // 子线程 使用 move关键字 转移所有权
    #[test]
    fn test_move() {
        let a = String::from("当前函数资源");
        let b = thread::spawn(move || {
            println!("子线程={}", a)
        });
        b.join().unwrap();
    }

    use std::sync::mpsc;
    
    /// 通道
    #[test]
    fn test_channel() {
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let a1 = 3.14;
            sender.send(a1).unwrap();
        });
        let receVal = receiver.recv().unwrap();
        println!("接收值={}", receVal)
    }
}
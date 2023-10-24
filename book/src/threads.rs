#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    #[test]
    fn test_spawn_and_join() {
        let handle = thread::spawn(|| {
            for i in 1..5 {
                println!("hi number {} from the spawned thread", i);
                thread::sleep(Duration::from_millis(i))
            }
        });

        match handle.join() {
            Ok(_) => assert!(true),
            Err(e) => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn test_move_ownership_to_a_thread() {
        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            v.iter()
                .for_each(|val| println!("vector item from the spawned thread: {}", val))
        });

        handle.join().unwrap();
    }
}

#[cfg(test)]
mod channel_tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_collect_from_a_thread() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let msg = String::from("Hello from child");
            tx.send(msg).unwrap();
        });

        let received = rx.recv().unwrap();

        assert_eq!(received, "Hello from child");
    }

    #[test]
    fn test_collect_all_from_threads() {
        let (tx1, rx) = mpsc::channel();
        let tx2 = tx1.clone();

        thread::spawn(move || {
            let msgs = vec![
                String::from("1: Hello"),
                String::from("1: from"),
                String::from("1: thread"),
            ];
            thread::sleep(Duration::from_millis(100));

            for msg in msgs {
                tx1.send(msg).unwrap();
            }
        });

        thread::spawn(move || {
            let msgs = vec![
                String::from("2: Hello"),
                String::from("2: from"),
                String::from("2: thread"),
            ];
            thread::sleep(Duration::from_millis(10));

            for msg in msgs {
                tx2.send(msg).unwrap();
            }
        });

        let mut result = vec![];
        for msg in rx {
            result.push(msg.clone());
        }

        assert_eq!(result.len(), 6);
    }
}

#[cfg(test)]
mod mutex_tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let c = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = c.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }
}

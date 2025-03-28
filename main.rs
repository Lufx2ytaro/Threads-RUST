use std::thread;
use std::time::Duration;

fn main() {
    // Создаём новый поток
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Поток: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let handle_two = thread::spawn(move|| {
      for i in 0..10 {
        println!("Потокище: {}",i);
        thread::sleep(Duration::from_millis(10));
      }
    });

    // Главный поток продолжает работать
    for i in 1..3 {
        println!("Главный поток: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Ждём завершения дочернего потока
    handle.join().unwrap();
    handle_two.join().unwrap();
}
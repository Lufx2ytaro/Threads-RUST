// use std::thread;
// use std::time::Duration;

// fn main() {
//     // Создаём новый поток
//     let handle = thread::spawn(|| {
//         for i in 1..5 {
//             println!("Поток: {}", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     let handle_two = thread::spawn(|| {
//       for i in 0..10 {
//         println!("Потокище: {}",i);
//         thread::sleep(Duration::from_millis(10));
//       }
//     });

//     // Главный поток продолжает работать
//     for i in 1..3 {
//         println!("Главный поток: {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     // Ждём завершения дочернего потока
//     handle.join().unwrap();
//     handle_two.join().unwrap();
// }


use std::sync::mpsc;
use std::thread;

fn main() {
    // Создаём канал
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    // Передаём tx в новый поток

    thread::spawn( move || {
        let msg = String::from("Привет из потока!");
        tx.send(msg).unwrap(); // Отправляем сообщение
    });

    thread::spawn( move || {
      let txt = String::from("Hello from thread");
      tx1.send(txt).unwrap();
    });

    // Получаем сообщение в главном потоке
    let received = rx.recv().unwrap();
    let cage = rx.recv().unwrap();
    println!("Получено: {}", received);
    println!("Catch: {}", cage);
}
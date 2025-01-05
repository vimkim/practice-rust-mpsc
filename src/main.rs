use rand::Rng;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

struct Stats {
    produced: usize,
    consumed: usize,
}

fn spawn_producer(tx: Sender<i32>, stats: Arc<Mutex<Stats>>, id: usize) {
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let num = rng.gen_range(1..100);
            match tx.send(num) {
                Ok(_) => {
                    let mut stats = stats.lock().unwrap();
                    stats.produced += 1;
                    println!("Producer {}: Generated {}", id, num);
                }
                Err(_) => break, // 채널이 닫히면 종료
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn spawn_consumer(rx: Arc<Mutex<Receiver<i32>>>, stats: Arc<Mutex<Stats>>, id: usize) {
    thread::spawn(move || {
        loop {
            let num = {
                let rx = rx.lock().unwrap();
                match rx.recv() {
                    Ok(n) => n,
                    Err(_) => break, // 채널이 닫히면 종료
                }
            };

            let result = num * num;
            {
                let mut stats = stats.lock().unwrap();
                stats.consumed += 1;
                println!("Consumer {}: Processed {} -> {}", id, num, result);
            }
        }
    });
}

fn main() {
    let n_producers = 3; // Producer 수
    let m_consumers = 2; // Consumer 수

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let stats = Arc::new(Mutex::new(Stats {
        produced: 0,
        consumed: 0,
    }));

    // Producer 스레드 생성
    let mut producer_handles = vec![];
    for i in 0..n_producers {
        let tx = tx.clone();
        let stats = Arc::clone(&stats);
        spawn_producer(tx, stats, i);
        producer_handles.push(());
    }
    drop(tx); // 원본 sender 드롭

    // Consumer 스레드 생성
    let mut consumer_handles = vec![];
    for i in 0..m_consumers {
        let rx = Arc::clone(&rx);
        let stats = Arc::clone(&stats);
        spawn_consumer(rx, stats, i);
        consumer_handles.push(());
    }

    // 10초 동안 실행
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        thread::sleep(Duration::from_millis(100));
    }

    // 최종 통계 출력
    let stats = stats.lock().unwrap();
    println!("\nFinal Statistics:");
    println!("Total messages produced: {}", stats.produced);
    println!("Total messages consumed: {}", stats.consumed);
}

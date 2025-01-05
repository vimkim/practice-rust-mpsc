use rand::Rng;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tracing::instrument;

#[derive(Debug)]
struct Stats {
    produced: usize,
    consumed: usize,
}

#[instrument(name = "producer", skip(tx, stats))]
fn spawn_producer(tx: Sender<i32>, stats: Arc<Mutex<Stats>>, id: usize) -> thread::JoinHandle<()> {
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
                Err(_) => {
                    println!("Producer {}: Channel closed, stopping", id);
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    })
}

#[instrument(name = "consumer", skip(rx, stats))]
fn spawn_consumer(
    rx: Arc<Mutex<Receiver<i32>>>,
    stats: Arc<Mutex<Stats>>,
    id: usize,
) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        let num = {
            let rx = rx.lock().unwrap();
            match rx.recv() {
                Ok(n) => n,
                Err(_) => {
                    println!("Consumer {}: Channel closed, stopping", id);
                    break;
                }
            }
        };

        let result = num * num;
        {
            let mut stats = stats.lock().unwrap();
            stats.consumed += 1;
            println!("Consumer {}: Processed {} -> {}", id, num, result);
        }
    })
}

fn main() {
    // Initialize tracing with debug level
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .init();

    let n_producers = 3;
    let m_consumers = 2;

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let stats = Arc::new(Mutex::new(Stats {
        produced: 0,
        consumed: 0,
    }));

    println!(
        "Starting with {} producers and {} consumers",
        n_producers, m_consumers
    );

    // Producer 스레드 생성
    let mut producer_handles = vec![];
    for i in 0..n_producers {
        let tx = tx.clone();
        let stats = Arc::clone(&stats);
        let handle = spawn_producer(tx, stats, i);
        producer_handles.push(handle);
    }
    drop(tx); // 원본 sender 드롭

    // Consumer 스레드 생성
    let mut consumer_handles = vec![];
    for i in 0..m_consumers {
        let rx = Arc::clone(&rx);
        let stats = Arc::clone(&stats);
        let handle = spawn_consumer(rx, stats, i);
        consumer_handles.push(handle);
    }

    println!("All threads initialized, running for 10 seconds");

    // 10초 동안 실행
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        thread::sleep(Duration::from_millis(100));
    }

    println!("\nTime elapsed, gathering final statistics");

    // 최종 통계 출력
    let stats = stats.lock().unwrap();
    println!("\nFinal Statistics:");
    println!("Total messages produced: {}", stats.produced);
    println!("Total messages consumed: {}", stats.consumed);
}

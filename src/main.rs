use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn ejercicio_11_fork_join() {
    println!("\n=== Ejercicio 11 - Fork-Join ===");

    let v: Vec<i32> = (1..=100).collect();
    let (izq, der) = v.split_at(50);

    thread::scope(|s| {
        let h1 = s.spawn(|| izq.iter().sum::<i32>());
        let h2 = s.spawn(|| der.iter().sum::<i32>());

        let total = h1.join().unwrap() + h2.join().unwrap();
        println!("Suma total = {}", total);
    });
}

async fn sensor_1() -> i32 {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("sensor_1 listo");
    42
}

async fn sensor_2() -> i32 {
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("sensor_2 listo");
    73
}

async fn ejercicio_12_async() {
    println!("\n=== Ejercicio 12 - Async/Await ===");

    let inicio = Instant::now();
    let (a, b) = tokio::join!(sensor_1(), sensor_2());
    let elapsed = inicio.elapsed();

    println!("Sensores: {} y {}", a, b);
    println!("Tiempo total: {:.2?}", elapsed);
}

fn ejercicio_13_productor_consumidor() {
    println!("\n=== Ejercicio 13 - Productor-Consumidor ===");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for n in 1..=10 {
            tx.send(n).unwrap();
        }
    });

    let mut suma = 0;
    while let Ok(n) = rx.recv() {
        suma += n;
    }

    println!("Suma total = {}", suma);
}

#[tokio::main]
async fn main() {
    println!("Actividad 6 - Modelos de Concurrencia");
    println!("Parte 5: Implementacion en Rust");

    ejercicio_11_fork_join();
    ejercicio_12_async().await;
    ejercicio_13_productor_consumidor();
}

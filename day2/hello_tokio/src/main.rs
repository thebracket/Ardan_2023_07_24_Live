use tokio::task::JoinSet;

async fn hello() {
    println!("Hello from async");
}

async fn double(n: i32) -> i32 {
    n * 2
}

async fn ticker() {
    for i in 0..100 {
        println!("tick {i}");
        tokio::task::yield_now().await;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(ticker());
    tokio::spawn(ticker());

    hello().await;
    let (first, second) = tokio::join!(double(2), double(3));
    println!("{first}, {second}");

    // Using Tokio JoinSet
    let mut set = JoinSet::new();
    for i in 0..10 {
        set.spawn(double(i));
    }
    while let Some(res) = set.join_next().await {
        println!("{res:?}");
    }
}

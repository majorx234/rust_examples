use futures::executor::block_on;

async fn future_world() {
    println!("Hallo future World");
}

fn main() {
    let myfuture = future_world();
    block_on(myfuture);
}

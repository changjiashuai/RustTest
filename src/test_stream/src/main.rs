use std::future::{Future};
use std::io;
use std::pin::Pin;
use futures::channel::mpsc;
use futures::{select, SinkExt, Stream, StreamExt, try_join};

fn bad() -> impl Future<Output=u8> {
    async {
        let x = 5;
        borrow_x(&x).await
    }
}

async fn borrow_x(x: &u8) -> u8 {
    *x
}

fn move_block() -> impl Future<Output=()> {
    let my_string = "foo".to_string();
    async move {
        println!("{my_string}")
    }
}

async fn send_recv() {
    const BUFFER_SIZE: usize = 10;
    let (mut tx, mut rx) = mpsc::channel::<i32>(BUFFER_SIZE);
    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    drop(tx);

    assert_eq!(Some(1), rx.next().await);
    assert_eq!(Some(2), rx.next().await);
    assert_eq!(None, rx.next().await);
}

async fn jump_around(
    mut stream: Pin<&mut dyn Stream<Item=Result<u8, io::Error>>>
) -> Result<(), io::Error> {
    use futures::stream::TryStreamExt;
    const MAX_CONCURRENT_JUMPERS: usize = 100;
    stream.try_for_each_concurrent(MAX_CONCURRENT_JUMPERS, |num| async move {
        //do something
        Ok(())
    }).await?;
    Ok(())
}

async fn enjoy_book() -> String {
    String::from("I'm read book")
}

async fn enjoy_music() -> String {
    String::from("I'm listen music")
}

async fn enjoy_book_and_music() -> (String, String) {
    let book_fut = enjoy_book();
    let music_fut = enjoy_music();
    futures::join!(book_fut, music_fut);
    try_join!(book_fut, music_fut);
    // ("".to_string(), "".to_string())

    select! {}
}

fn main() {
    // bad();
    // move_block();
    // send_recv();
    enjoy_book_and_music();
}

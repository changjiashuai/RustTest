use futures::executor::block_on;

fn main() {
    let future = do_something();
    block_on(future);

    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());

    block_on(async_main());
}

async fn learn_and_sing(){
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main(){
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

struct Song {
    author:String,
    name:String,
}

async fn learn_song() -> Song{
    Song{
        author:"A".to_string(),
        name:String::from("from A"),
    }
}

async fn sing_song(song:Song){
    println!("I'll sing {} song {} from {}", song.name, "lalalalala", song.author);
}

async fn dance(){
    println!("I'm dance")
}

async fn do_something() {
    hello_cat().await;
    println!("go go go !");
}

async fn hello_cat(){
    println!("hello, kitty!");
}
// Rust does not supoort async for loops. instead iterating strems using while let paired with StreamExt::next()

// streams asynchronous series of values like std::iter::Iterator

use tokio_stream::StreamExt;
//StreamExt trait used to asynchronously iterate over the stream with next().await

// #[tokio::main]

// async fn main() {
//     // stream is asynchronous series of value. tokio_stream::iter is crate a stream over an iterator
//     // `tokio_stream::iter` is called an adapter convert the series of value into stream
//     let  mut stream = tokio_stream::iter(&[1,2,3]);
//     // '_ is an elided lifetime, which means the lifetime is infreed by the copiler.
//     // it's a shordhand for saying that the iterator borrows from the original slice.
//     // lifetime of the iterators is inferred from the lifetime of the original slice.
//     // Option<T> is return by the stream.next() , here  T is value of stream type at the last stream.next() return None
//     // mut is need here because stream cahnge over time here stream.next().await
//     while let Some(v) = stream.next().await  {
//         println!("GOT = {:?}", v);
//     }
// }


use mini_redis::client;

async fn publish() -> mini_redis::Result<()> {
    let addr =  "127.0.0.1:6379";
    // redis server located at `172.0.0.1` and port `6379`
    let mut client = client::connect(addr).await?;
    // client::connect is part of miniredis that handle the connection logic

    // publish some data or boradcast data
    // client publish message through the channel numbers with the messages.
    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "2".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;
    Ok(())

}
// listening message subscribe like a radio receiver in a car or home
async fn subscribe() -> mini_redis::Result<()>{
    let client = client::connect("127.0.0.1:6379").await?;

    let subscribe = client.subscribe(vec!["numbers".to_string()]).await?;
    // tune a channel numbers using like a mechanisom `subscribe.into_stream()`
    let message = subscribe.into_stream();
    tokio::pin!(message);
    // it is like special kind of mechanizom that prevent the freequency dirverions.

    while let Some(msg) = message.next().await {
        println!("got = {:?}", msg)
    }

    Ok(())

}

#[tokio::main]

async fn main() -> mini_redis::Result<()>{
    tokio::spawn(async {
        publish().await
    });
    subscribe().await?;
    println!("DONE");
    Ok(())
}
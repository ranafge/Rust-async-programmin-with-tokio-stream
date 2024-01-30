// Rust does not supoort async for loops. instead iterating strems using while let paired with StreamExt::next()

// streams asynchronous series of values like std::iter::Iterator

use tokio::task::yield_now;
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
    let addr = "127.0.0.1:6379";
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
async fn subscribe() -> mini_redis::Result<()> {
    let client = client::connect("127.0.0.1:6379").await?;

    let subscribe = client.subscribe(vec!["numbers".to_string()]).await?;
    // tune a channel numbers using like a mechanisom `subscribe.into_stream()`
    // let message = subscribe.into_stream();
    // using a take adapter and then done
    // let messages = subscribe.into_stream().take(3);
    // filter only singel digit message using filter adapter with match statement.
    let messages = subscribe
        .into_stream()
        .filter(|msg| match msg {
            Ok(msg) if msg.content.len() == 1 => true,
            _ => false,
        })
        .take(3)
        .map(|msg| msg.unwrap().content);
    // using filter_map  adapter

    tokio::pin!(messages);
    // it is like special kind of mechanizom that prevent the freequency dirverions.

    while let Some(msg) = messages.next().await {
        println!("got = {:?}", msg)
    }

    Ok(())
}

// #[tokio::main]

// async fn main() -> mini_redis::Result<()> {
//     tokio::spawn(async { publish().await });
//     subscribe().await?;
//     println!("DONE");
//     Ok(())
// }

// Adapters function take a stream and return another stream are often called stream adapter.

// Implementing Stream trait

/*
    stream trait is very similar to the Future trait
*/

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

/*
    Stream trait ekta postbox er moto jekhane somoyer shathe shathe akash letter pathabe ami post office
    theke letter receive karbo.

    poll_next() ekta postbox check karar moto, akhash letter pathaise check kara postbox
    a chithi poiseki na.

    context and waker are like megical notification ja executor ke janabe when might be
    new letter and check it again.

    size_hint() as estimating letter count - estimating number of expected letters.

    Delay future like scheduling letters to be delivered at specific intervals.
*/
pub trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}



use std::time::{Duration, Instant};
#[derive(Debug)]
struct Delay {
    when: Instant,
}
impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<&'static str> {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {
            // Ignore this line for now.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
struct Interval {
    rem: usize,
    delay: Delay,
}

impl Interval {
    fn new() -> Self {
        Self {
            rem: 3,
            delay: Delay {
                when: Instant::now(),
            },
        }
    }
}

impl Stream for Interval {
    type Item = ();

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.rem == 0 {
            return Poll::Ready(None);
        }
        // Pin::new takes a pointer as argument that is a future
        match Pin::new(&mut self.delay).poll(cx) {
            Poll::Ready(_) => {
                let when = self.delay.when + Duration::from_millis(10);
                self.delay = Delay { when };
                self.rem -= 1;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}


use async_stream::stream;
// #[tokio::main]
// async fn main() {
//     let mut when = Instant::now();
//     for _ in 0..3 {
//         let delay = Delay {when};
//         delay.await;
//         yield_now();

//         when += Duration::from_millis(10);
//     }
// }

// #[tokio::main]
// async fn main() {
//     let mut when = Instant::now();
//     for _ in 0..10{
//         let delay = Interval {
//             rem: 10,
//             delay: Delay { when }
//         };
//         delay.delay.await;
//         yield_now().await;
//         when += Duration::from_millis(10);
//     }
// }


#[tokio::main]
async fn main() {
    use async_stream::stream;
    use std::time::{Duration, Instant};
    
   let x =  stream! {
        let mut when = Instant::now();
        for _ in 0..3 {
            let delay = Delay { when };
            delay.await;
            yield ();
            when += Duration::from_millis(10);
        }
    };
   let total_stream = x.take(2);
   tokio::pin!(total_stream);
   // stream is asynchrous sequentional value like iterator. So use while let instead of for loop. note that for loop is supported stream
   while let Some(_) = total_stream.next().await  {
       println!("Receive stream");
   }
   
}

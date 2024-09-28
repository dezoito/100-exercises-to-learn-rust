/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?
// use std::sync::mpsc;
use tokio::sync::mpsc;

pub struct Message {
    payload: String,
    response_channel: mpsc::Sender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        // * changed to Some(msg) and await the result
        if let Some(msg) = receiver.recv().await {
            println!("Pong received: {}", msg.payload);
            // * channel now needs an u8 arg
            let (sender, new_receiver) = mpsc::channel(1);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .await // * we have to await now as this is async
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    // use std::sync::mpsc;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = mpsc::channel(1);
        // * changed receiver to mut here + channel needs an u8 arg
        let (response_sender, mut response_receiver) = mpsc::channel(1);
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .await // * added await cousa this is async now
            .unwrap();

        tokio::spawn(pong(receiver));

        // * added await cousa this is async now
        let answer = response_receiver.recv().await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}

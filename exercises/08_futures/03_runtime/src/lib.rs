// TODO: Implement the `fixed_reply` function. It should accept two `TcpListener` instances,
//  accept connections on both of them concurrently, and always reply clients by sending
//  the `Display` representation of the `reply` argument as a response.
// Import necessary traits and types for formatting and network communication.
use std::fmt::{format, Display}; // Display trait is used to convert the 'reply' into a human-readable string.
use std::sync::Arc; // Arc (Atomic Reference Counted) allows safe concurrent access to shared data.
use tokio::io::AsyncWriteExt; // Provides asynchronous write functionality.
use tokio::net::TcpListener; // TcpListener allows for asynchronous TCP connections.

// The main async function that accepts two TcpListeners and a reply object.
pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    // T must implement the Display trait so it can be formatted into a string.
    // It must also be Send (safe to transfer ownership across threads),
    // Sync (safe to share between threads), and 'static (lives for the duration of the program).
    T: Display + Send + Sync + 'static,
{
    // To handle the fact that 'reply' cannot be cloned, we wrap it in an Arc (atomic reference counting),
    // which enables multiple tasks to share ownership of the same data safely.
    let reply = Arc::new(reply);

    // Spawn two concurrent tasks for handling connections on both TcpListeners.
    // Each task will call the `_fixed_reply` function to listen and reply to clients.
    let handle1 = tokio::spawn(_fixed_reply(first, Arc::clone(&reply))); // Arc::clone creates another reference to the shared reply object.
    let handle2 = tokio::spawn(_fixed_reply(second, reply)); // The second task also gets the shared reply object.

    // Await both tasks to run concurrently and complete. `tokio::join!` waits for both tasks to finish.
    tokio::join!(handle1, handle2);
}

// Private async function that accepts connections and replies to each client.
// It runs inside the tasks spawned by the main `fixed_reply` function.
async fn _fixed_reply<T>(listener: TcpListener, reply: Arc<T>)
where
    // The same trait bounds as the main function: T must implement Display, Send, Sync, and 'static.
    T: Display + Send + Sync + 'static,
{
    // An infinite loop to continuously accept incoming connections on the TcpListener.
    loop {
        // Await and accept a new connection. 'listener.accept()' returns a new socket.
        let (mut socket, _) = listener.accept().await.unwrap();

        // Split the socket into a reader and a writer part. We don't need the reader part,
        // but we use the writer part to send a response back to the client.
        let (mut _reader, mut writer) = socket.split();

        // Write the formatted 'reply' to the socket. The 'format!()' macro is used to create
        // a string from the Display implementation of 'reply'. The string is then converted
        // to bytes and written asynchronously to the client using 'write_all()'.
        writer
            .write_all(format!("{}", reply).as_bytes())
            .await
            .unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();

                    // Read the response
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}

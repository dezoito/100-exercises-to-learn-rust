use tokio::io::copy;
use tokio::net::TcpListener;

// TODO: write an echo server that accepts incoming TCP connections and
//  echoes the received data back to the client.
//  `echo` should not return when it finishes processing a connection, but should
//  continue to accept new connections.
//
// Hint: you should rely on `tokio`'s structs and methods to implement the echo server.
// In particular:
// - `tokio::net::TcpListener::accept` to process the next incoming connection
// - `tokio::net::TcpStream::split` to obtain a reader and a writer from the socket
// - `tokio::io::copy` to copy data from the reader to the writer
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    // * Asynchronously wait for an inbound socket.
    // * Explanation: https://chatgpt.com/c/28d664e9-c91a-40ad-be39-4e13d8d9a79e
    loop {
        let (mut sock, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut reader, mut writer) = sock.split();
            // * `copy` is an asynchronous function that continuously
            // * reads data from the `reader` and writes it to the `writer`.”
            copy(&mut reader, &mut writer).await.unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut reader, mut writer) = socket.split();

            // Send the request
            writer.write_all(request.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer.shutdown().await.unwrap();

            // Read the response
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}

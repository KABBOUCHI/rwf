use std::marker::Unpin;
use std::sync::Arc;
use tokio::io::{AsyncRead, AsyncReadExt};

use super::{Error, Head};

#[derive(Debug, Clone)]
pub struct Request {
    head: Arc<Head>,
    body: Arc<Vec<u8>>,
}

impl Request {
    pub async fn read(mut stream: impl AsyncRead + Unpin) -> Result<Self, Error> {
        let head = Arc::new(Head::read(&mut stream).await?);
        let content_length = head.content_length().unwrap_or(0);
        let mut body = vec![0u8; content_length];
        stream
            .read_exact(&mut body)
            .await
            .map_err(|_| Error::MalformedRequest("incorrect content length"))?;

        Ok(Request {
            head,
            body: Arc::new(body),
        })
    }

    pub fn head(&self) -> &Head {
        &self.head
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_response() {
        let body = ("GET / HTTP/1.1\r\n".to_owned()
            + "Content-Type: application/json\r\n"
            + "Accept: */*\r\n"
            + "Content-Length: 4\r\n"
            + "\r\n"
            + "hello")
            .as_bytes()
            .to_vec();
        let response = Request::read(&body[..]).await.expect("response");
        println!("{:?}", response);
    }
}

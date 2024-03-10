use crate::{codes::ByteCode, transfer::transfer::RequestTran};
use bytes::BytesMut;
use protobuf::Message;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_util::codec::Encoder;

pub struct RequestEncode {
    pub stream: TcpStream,
    pub transfer: RequestTran,
}

impl Encoder<Vec<u8>> for RequestEncode {
    type Error = std::io::Error;
    fn encode(&mut self, item: Vec<u8>, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // 如果字符串大于8m，则不要发送，
        // 接受。
        if item.len() > ByteCode::Max.size() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", item.len()),
            ));
        }

        // 将长度转换为字节数组。
        // 由于上面的长度检查，转换为 u32 不会溢出。
        let len_slice = u32::to_le_bytes(item.len() as u32);

        // Reserve space in the buffer.
        dst.reserve(4 + item.len());

        // Write the length and string to the buffer.
        dst.extend_from_slice(&len_slice);
        dst.extend_from_slice(&item);
        Ok(())
    }
}

impl RequestEncode {
    pub async fn send(&mut self) {
        let mut buf = BytesMut::new();
        let bytes = self.transfer.write_to_bytes().unwrap();
        self.encode(bytes, &mut buf).unwrap();
        // Write some data.
        self.stream.write_all(&buf).await.unwrap();
    }
}

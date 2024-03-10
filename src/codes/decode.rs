use crate::codes::ByteCode;
use crate::transfer::transfer::RequestTran;
use bytes::Buf;
use protobuf::Message;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio_util::codec::Decoder;

pub struct ResponseDecode {
    pub stream: TcpStream,
    pub transfer: Option<RequestTran>,
}

impl Decoder for ResponseDecode {
    type Item = RequestTran;

    type Error = std::io::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            // Not enough data to read length marker.
            return Ok(None);
        }
        // Read length marker.
        let mut length_bytes = [0u8; 4];
        length_bytes.copy_from_slice(&src[..4]);
        let length = u32::from_le_bytes(length_bytes) as usize;
        // 检查长度不要太大，避免拒绝
        // 服务器内存不足的服务攻击。
        if length > ByteCode::Max.size() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Frame of length {} is too large.", length),
            ));
        }

        if src.len() < 4 + length {
            // 完整的字符串尚未到达。
            // 我们在缓冲区中保留更多空间。这并不严格
            // 必要的，但从性能角度来说是个好主意。
            src.reserve(4 + length - src.len());

            // 我们通知 Framed 我们需要更多字节来形成下一个
            // 框架。
            return Ok(None);
        }

        // 使用 advance 修改 src 使其不再包含
        // 这个帧
        let data = src[4..4 + length].to_vec();
        src.advance(4 + length);

        let request_tran = RequestTran::parse_from_bytes(&data).unwrap();
        Ok(Some(request_tran))
    }
}

impl ResponseDecode {
    pub async fn rece(&mut self) {
        let mut buf = bytes::BytesMut::new();
        match self.stream.read_buf(&mut buf).await {
            // socket closed
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };
        let date = self.decode(&mut buf);
        let date = match date {
            Ok(date) => {
                if let Some(date) = date {
                    date
                } else {
                    RequestTran::new()
                }
            }
            Err(_) => {
                panic!("error")
            }
        };
        println!("{:#?}", date);
    }
}

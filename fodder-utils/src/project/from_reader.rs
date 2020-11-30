use tokio::prelude::*;
use serde::Deserialize;


#[async_trait::async_trait]
pub trait FromReader: for<'de> Deserialize<'de> + Sized {
    const CAPACITY: usize;

    async fn from_reader<R>(reader: &mut R) -> crate::Result<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let mut bytes = Vec::<u8>
            ::with_capacity(Self::CAPACITY);
        reader.read_to_end(&mut bytes)
            .await?;
        let result: Self = serde_json
            ::from_reader(&bytes[..])?;
        Ok(result)
    }
}

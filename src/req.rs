use std::io;
use std::io::Read;
use serde::de::Unexpected::Str;
use ureq::Response;
const INTO_STRING_LIMIT: usize = 256 * 1_024 * 1_024;

pub fn into_string(r: Response) -> io::Result<String> {
    #[cfg(feature = "charset")]
        let encoding = Encoding::for_label(self.charset().as_bytes())
        .or_else(|| Encoding::for_label(DEFAULT_CHARACTER_SET.as_bytes()))
        .unwrap();

    let mut buf: Vec<u8> = vec![];
    r.into_reader()
        .take((INTO_STRING_LIMIT + 1) as u64)
        .read_to_end(&mut buf)?;
    if buf.len() > INTO_STRING_LIMIT {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "response too big for into_string",
        ));
    }
    #[cfg(not(feature = "charset"))]
    {
        Ok(String::from_utf8_lossy(&buf).to_string())
    }
}
pub fn into_bytes(r: Response) -> io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];
    r.into_reader()
        .take((INTO_STRING_LIMIT + 1) as u64)
        .read_to_end(&mut buf)?;
    Ok(buf)
}
pub fn request_get_bytes(url: String) -> Option<Vec<u8>> {
    let resp = reqwest::blocking::get(url);
    return match resp {
        Ok(response) => {
            let mut buf: Vec<u8> = vec![];
            let bytes_result = response.bytes();
            match bytes_result {
                Ok(bytes) => {
                    let take = bytes.take((INTO_STRING_LIMIT + 1) as u64)
                        .read_to_end(&mut buf);
                    match take {
                        Ok(_) => { Some(buf) }
                        Err(_) => { None }
                    }
                }
                Err(_) => { None }
            }
        }
        Err(_) => {
            None
        }
    }
}
pub fn request_get(url: &str) -> Option<String> {
    match ureq::get(url).call() {
        Ok(response) => Some(into_string(response).unwrap()),

        _ => { return None }
    }
}
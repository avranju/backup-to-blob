use std::io::{Result, Write};

use super::AzureStorageOpt;

const AD_CLIENT_SECRET: &'static str = "Rwe06itW57fNasCPwV-Yi:SaNO.QKAL=";

pub struct BlobClient<'a> {
    storage: &'a AzureStorageOpt,
    file_name: &'a str,
}

impl<'a> BlobClient<'a> {
    pub fn new(storage: &'a AzureStorageOpt, file_name: &'a str) -> Self {
        BlobClient { storage, file_name }
    }
}

impl<'a> Write for BlobClient<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        println!("  BlobClient::write() - Writing {} bytes", buf.len());

        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        println!("BlobClient::flush()");
        Ok(())
    }
}

//fn string_to_sign(
//    verb: &str,
//    content_encoding: Option<&str>,
//    content_language: Option<&str>,
//    content_length: Option<&str>,
//    content_md5: Option<&str>,
//    content_type: Option<&str>,
//    date: Option<&str>,
//    if_modified_since: Option<&str>,
//    if_match: Option<&str>,
//    if_none_match: Option<&str>,
//    if_unmodified_since: Option<&str>,
//    range: Option<&str>,
//    headers: &HeaderMap,
//) -> String {
//    "".to_owned()
//}

//fn canonicalize_headers(headers: &HeaderMap) -> String {
//    let mut v = headers
//        .iter()
//        .map(|(key, value)| (key.as_str().to_lowercase(), value))
//        .filter(|(key, _)| key.starts_with("x-ms-"))
//        .collect::<Vec<_>>();
//    v.sort_unstable_by_key(|(key, _)| key.clone());
//
//    let v = v
//        .iter()
//        .map(|(key, value)| format!("{}:{}", key.as_str(), value.to_str().unwrap()))
//        .collect::<Vec<String>>();
//
//    v.join("\n")
//}

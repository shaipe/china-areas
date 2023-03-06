//! copyright © ecdata.cn 2020-persent
//! 同步网络请求客户端

use crate::{Error, Result};
use bytes::Bytes;
use reqwest::blocking::Client as HttpClient;
use reqwest::header;
use serde::Serialize;
use std::time::Duration;

const DEFAULT_USER_AGENT: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3534.4 Safari/537.36";

/// 请求客户端
#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) client: HttpClient,
    charset: String,
}

impl Client {
    /// 创建客户端访问对象
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(DEFAULT_USER_AGENT),
        );

        Client {
            client: HttpClient::builder()
                .timeout(Duration::from_secs(300))
                .connect_timeout(Duration::from_secs(300))
                .default_headers(headers)
                .build()
                .unwrap(),
            charset: "utf-8".to_owned(),
        }
    }

    /// 设置获取数据的编码方式
    pub fn set_charset(&mut self, charset: &str) -> &Self {
        self.charset = charset.to_owned();
        self
    }

    /// post方式提交数据
    /// url:
    /// param:
    pub fn post<T: Serialize + ?Sized>(&self, url: &str, params: &T) -> Result<String> {
        match self.client.post(url).json(params).send() {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text() {
                        Ok(txt) => {
                            // println!("--- {} ----", txt);
                            Ok(txt)
                        }
                        Err(e) => Err(Error::custom(-1, format!("Send request error: {}", e))),
                    }
                } else {
                    Err(Error::custom(500, format!("status={}", res.status())))
                }
            }
            Err(e) => Err(Error::custom(500, format!("Send request error: {}", e))),
        }
    }

    /// 发送二进制文件
    pub fn post_betyes(&self, url: &str, body: Bytes) -> Result<String> {
        match self.client.post(url).body(body.to_vec()).send() {
            Ok(res) => {
                if res.status() == 200 {
                    match res.text() {
                        Ok(txt) => Ok(txt),
                        Err(e) => Err(Error::custom(-1, format!("Send request error: {}", e))),
                    }
                } else {
                    Err(Error::custom(500, format!("status={}", res.status())))
                }
            }
            Err(e) => Err(Error::custom(500, format!("Send request error: {}", e))),
        }
    }

    /// get方法
    #[allow(dead_code)]
    pub fn get(&self, url: &str) -> Result<String> {
        match self.client.get(url).send() {
            Ok(res) => {
                if res.status() == 200 {
                    let tres = if self.charset.len() > 0 {
                        res.text_with_charset(&self.charset)
                    } else {
                        res.text()
                    };
                    match tres {
                        Ok(txt) => Ok(txt),
                        Err(e) => Err(Error::custom(-1, format!("Send request error: {}", e))),
                    }
                } else {
                    Err(Error::custom(500, format!("status={}", res.status())))
                }
            }
            Err(e) => Err(Error::custom(500, format!("Send request error: {}", e))),
        }
    }

    ///
    #[inline]
    pub fn json_decode(&self, data: &str) -> Result<serde_json::Value> {
        let obj: serde_json::Value = match serde_json::from_str(data) {
            Ok(decoded) => decoded,
            Err(ref e) => {
                return Err(Error::custom(-3, format!("Json decode error: {}", e)));
            }
        };
        let code = match obj["Code"].as_i64() {
            Some(v) => v,
            None => 500,
        };
        if code != 200 {
            let msg: String = obj["Message"].to_string();
            return Err(Error::custom(code as i32, msg));
        }
        Ok(obj)
    }
}

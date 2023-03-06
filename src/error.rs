//! copyright © ecdata.cn 2020 - present
//! 自定义错误信息处理
//! created by shaipe

use std::convert::Into;
use std::error::Error as StdError;
use std::fmt;


#[derive(Debug, PartialEq, Eq)]
pub enum LogLevel {
    // 信息输出
    Info,
    // 错误
    Error,
    // 警告
    Warn,
    // 监控
    Watch,
}

impl LogLevel {
    /// 获取等级的字符信息
    pub fn as_str(&self) -> &str {
        match *self {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Watch => "watch",
            _ => "info",
        }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    Msg(String),
    Io(::std::io::Error),
    Custom { code: i32, msg: String },
}

/// The Error type
#[derive(Debug)]
pub struct Error {
    /// Kind of error
    pub kind: ErrorKind,
    pub source: Option<Box<dyn StdError>>,
}
unsafe impl Sync for Error {}
unsafe impl Send for Error {}

/// 继承标准接口
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        let source = self.source.as_ref().map(|c| &**c);
        if source.is_none() {
            match self.kind {
                // ErrorKind::Custom(ref err) => source = err.source(),
                _ => (),
            };
        }

        source
    }
}

/// 格式化显示设置
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Msg(ref message) => write!(f, "{}", message),
            ErrorKind::Custom { code, ref msg } => {
                write!(f, "custom error code: {}, message: {}", code, msg)
            }
            ErrorKind::Io(ref e) => write!(f, "{}", e),
        }
    }
}

impl Error {
    /// 获取错误代码
    pub fn get_code(&self) -> i32 {
        match self.kind {
            ErrorKind::Msg(ref _message) => 3000,
            ErrorKind::Custom { code, ref msg } => {
                let _ = msg;
                code
            }
            ErrorKind::Io(ref _e) => 4000,
        }
    }

    /// 获取错误中的消息
    pub fn get_message(&self) -> String {
        match self.kind {
            ErrorKind::Msg(ref msg) => msg.to_string(),
            ErrorKind::Custom { code, ref msg } => {
                let _ = code;
                msg.to_string()
            }
            ErrorKind::Io(ref e) => format!("{}", e),
        }
    }

    /// Creates generic error
    pub fn msg(value: impl ToString) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: None,
        }
    }

    /// Creates generic error with a cause
    pub fn chain(value: impl ToString, source: impl Into<Box<dyn StdError>>) -> Self {
        Self {
            kind: ErrorKind::Msg(value.to_string()),
            source: Some(source.into()),
        }
    }

    /// 自定义错误
    pub fn custom(code: i32, msg: impl ToString) -> Self {
        Self {
            kind: ErrorKind::Custom {
                code,
                msg: format!("{}", msg.to_string()),
            },
            source: None,
        }
    }

    // pub fn error_log(code: i32, msg: impl ToString, log: impl ToString) -> Self {
    //     // 将错误信息的日志记录下来
    //     Self::write_to_file(
    //         LogLevel::Error,
    //         format!("error: {}; log: {}", msg.to_string(), log.to_string()),
    //     );
    //     Self {
    //         kind: ErrorKind::Custom {
    //             code,
    //             msg: format!("{}", msg.to_string()),
    //         },
    //         source: None,
    //     }
    // }

    /// 自定义错误
    pub fn custom_err(code: i32, msg: impl ToString, source: impl Into<Box<dyn StdError>>) -> Self {
        Self {
            kind: ErrorKind::Custom {
                code,
                msg: msg.to_string(),
            },
            source: Some(source.into()),
        }
    }

    /// 获取错误跟踪信息
    pub fn trace_info() -> String {
        return "".to_owned();
        // 下面的在此处的宏中是没有意义的
        // let file_paths = file!()
        //     .split("/src/")
        //     .map(|s| s.to_owned())
        //     .collect::<Vec<_>>();
        // let file_path = if file_paths.len() > 1 {
        //     file_paths[1].replace(".rs", "")
        // } else {
        //     "".to_owned()
        // };
        // format!("{}::{}:{}", module_path!(), file_path, line!())
    }

    
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Self::msg(e)
    }
}
impl From<String> for Error {
    fn from(e: String) -> Self {
        Self::msg(e)
    }
}
impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Self {
        Self {
            kind: ErrorKind::Io(e),
            source: None,
        }
    }
}


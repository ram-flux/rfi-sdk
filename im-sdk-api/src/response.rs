#[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Response<T> {
    pub code: u32,
    pub message: String,
    pub result: Option<T>,
}

impl<T> From<Result<T, crate::Error>> for Response<T>
where
    T: serde::Serialize + Sized,
{
    fn from(res: Result<T, crate::Error>) -> Self {
        match res {
            Ok(ok) => ok.into(),
            Err(err) => {
                let (code, message) = err.into();
                Response {
                    code,
                    message,
                    result: None,
                }
            }
        }
    }
}

impl<T> From<T> for Response<T>
where
    T: serde::Serialize + Sized,
{
    fn from(msg: T) -> Self {
        Self {
            code: 200,
            message: String::new(),
            result: Some(msg),
        }
    }
}

impl From<crate::Error> for (u32, String) {
    fn from(err: crate::Error) -> Self {
        let (code, message) = match err {
            crate::Error::Parse(_) => (203, err.to_string()),
            crate::Error::BadRequest(_) => (203, err.to_string()),
            crate::Error::Database(_) => (203, err.to_string()),
            crate::Error::Wallet(_) => (203, err.to_string()),
            crate::Error::System(_) => (500, err.to_string()),
            crate::Error::UnAuthorize => (401, err.to_string()),
        };
        (code, message)
    }
}

impl<T> std::ops::FromResidual<Result<std::convert::Infallible, crate::Error>> for Response<T> {
    fn from_residual(residual: Result<std::convert::Infallible, crate::Error>) -> Self {
        match residual {
            Err(err) => {
                let (code, message) = err.into();
                Response {
                    code,
                    message,
                    result: None,
                }
            }
            Ok(_) => panic!("Infallible"),
        }
    }
}

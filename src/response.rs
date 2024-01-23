#[derive(serde::Serialize, serde::Deserialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct Response<T> {
    pub code: u32,
    #[serde(rename = "type")]
    typ: String,
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
                let (code, typ, message) = err.into();
                Response {
                    code,
                    typ,
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
            typ: "success".to_string(),
            message: String::new(),
            result: Some(msg),
        }
    }
}

impl From<crate::Error> for (u32, String, String) {
    fn from(err: crate::Error) -> Self {
        // use crate::Error;
        let (code, typ, message) = match err {
            // Error::Json(_) => (203, "json error".to_string(), err.to_string()),
            // Error::System(e) => e.into(),
            // Error::Route(_) => (201, "route error".to_string(), err.to_string()),
        };
        (code, typ, message)
    }
}

impl<T> std::ops::FromResidual<Result<std::convert::Infallible, crate::Error>> for Response<T> {
    fn from_residual(residual: Result<std::convert::Infallible, crate::Error>) -> Self {
        match residual {
            Err(err) => {
                let (code, typ, message) = err.into();
                Response {
                    code,
                    typ,
                    message,
                    result: None,
                }
            }
            Ok(_) => panic!("Infallible"),
        }
    }
}

use hyper::StatusCode;

pub(crate) struct ApiError {
    pub error: String,
    pub status: StatusCode,
}

pub(crate) trait ToApiError {
    type Value;
    fn into_api(self, code: StatusCode) -> Result<Self::Value, ApiError>;
}

impl<T, E> ToApiError for Result<T, E>
where
    E: std::error::Error,
{
    type Value = T;
    fn into_api(self, code: StatusCode) -> Result<T, ApiError> {
        self.map_err(|e| ApiError {
            error: e.to_string(),
            status: code,
        })
    }
}

use salvo::{
    async_trait, hyper::StatusCode, prelude::EndpointOutRegister, writing::Json, Depot, Request,
    Response, Writer,
};
use serde::Serialize;

use crate::app_error::AppError;

pub struct AppResponse<T>(pub AppResult<T>);

#[async_trait]
impl<T: Serialize + Default + Send> Writer for AppResponse<T> {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self.0 {
            Ok(data) => ResponseBuilder::with_data(data).into_response(res),
            Err(e) => e.write(req, depot, res).await,
        }
    }
}

impl<T: Serialize + Default + Send> EndpointOutRegister for AppResponse<T> {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation
            .responses
            .insert("0".to_string(), salvo::oapi::Response::new("success"));
        operation
            .responses
            .insert("500".to_string(), salvo::oapi::Response::new("error"));
    }
}

impl<T> From<AppResult<T>> for AppResponse<T> {
    fn from(result: AppResult<T>) -> Self {
        AppResponse(result)
    }
}

impl<T> From<AppError> for AppResponse<T> {
    fn from(result: AppError) -> Self {
        AppResponse(Err(result))
    }
}

#[derive(Debug, Serialize, Default)]
pub struct ResponseBuilder<T> {
    pub code: i32,
    pub data: T,
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponseBuilder {
    pub code: i32,
    pub msg: String,
    #[serde(skip)]
    pub source_error: AppError,
}

impl<T: Serialize + Send + Default> ResponseBuilder<T> {
    pub fn with_data(data: T) -> Self {
        Self {
            code: 0,
            data,
            msg: "success".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn with_data_msg(data: T, msg: &str) -> Self {
        Self {
            code: 0,
            data,
            msg: msg.to_string(),
        }
    }
}

impl ErrorResponseBuilder {
    pub fn with_err(err: AppError) -> Self {
    let (code, msg) = match &err {
    AppError::AnyHow(e) => (500, e.to_string()),
    AppError::ParseError(e) => (400, e.to_string()),
    AppError::RbatisErr(e) => (500, e.to_string()),
    AppError::ValidationError(e) => (400, e.to_string()),
    };
        Self {
            code,
            msg,
            source_error: err,
        }
    }
}
impl<T: Serialize + Send + Default> ResponseBuilder<T> {
    pub fn into_response(self, res: &mut Response) {
        res.render(Json(self));
    }
}

impl ErrorResponseBuilder {
    pub fn into_response(self, res: &mut Response) {
    let status_code = match self.source_error {
        AppError::AnyHow(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::ParseError(_) => StatusCode::BAD_REQUEST,
        AppError::RbatisErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
        AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
    };        
    res.stuff(status_code, Json(self));
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        ErrorResponseBuilder::with_err(self).into_response(res)
    }
}

impl EndpointOutRegister for AppError {
    fn register(_components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        operation
            .responses
            .insert("500".to_string(), salvo::oapi::Response::new("error"));
    }
}

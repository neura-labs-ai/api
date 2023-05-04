use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error,
};
use futures_util::future::LocalBoxFuture;

use crate::AppState;

pub struct AuthHandler;

impl<S: 'static, B> Transform<S, ServiceRequest> for AuthHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggingMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct LoggingMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<S>,
}

pub const AUTH_HEADER: &str = "Authorization";

struct ApiToken {
    exist: bool,
    value: String
}

impl<S, B> Service<ServiceRequest> for LoggingMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let svc = self.service.clone();

        // Get the app state
        let data = req.app_data::<web::Data<AppState>>().unwrap().clone();

        // get the request headers and check if the api key is present
        let token = match req.headers().get(AUTH_HEADER) {
            Some(t) => ApiToken {
                exist: true,
                value: t.to_str().unwrap().to_string(),
            },
            None => ApiToken {
                exist: false,
                value: String::from(""),
            }
        };

        Box::pin(async move {
            let res = svc.call(req).await?;

            if !token.exist {
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized Request!"));
            }

            // Used for internal api request from other systems.
            // Any request with the super key will be accepted.
            if token.value == crate::utils::env().unwrap().super_key {
                return Ok(res);
            }
            
            let authorized_request = data.db.has_api_key(token.value).await.unwrap();

            if !authorized_request {
                return Err(actix_web::error::ErrorUnauthorized("Unauthorized Request!"));
            }

            // everything is fine, return the response
            Ok(res)
        })
    }
}
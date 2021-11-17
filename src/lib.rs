#![allow(clippy::type_complexity)]

use axum::{
    body::{Body, BoxBody},
    handler::Handler,
    http::{Request, Response},
    routing::{delete, get, on, post, MethodFilter},
    Router,
};
use std::convert::Infallible;
use tower_service::Service;

pub trait RouterExt<B> {
    fn resource<F>(self, path: &str, f: F) -> Self
    where
        F: FnOnce(Resource<B>) -> Resource<B>;
}

impl<B> RouterExt<B> for Router<B> {
    fn resource<F>(self, path: &str, f: F) -> Self
    where
        F: FnOnce(Resource<B>) -> Resource<B>,
    {
        f(Resource {
            path: path.to_owned(),
            router: self,
        })
        .router
    }
}

pub struct Resource<B = Body> {
    path: String,
    router: Router<B>,
}

impl<B: Send + 'static> Resource<B> {
    fn index_create_path(&self) -> String {
        format!("/{}", self.path)
    }

    fn show_update_destroy_path(&self) -> String {
        format!("/{0}/:{0}_id", self.path)
    }

    fn route<T>(mut self, path: &str, svc: T) -> Self
    where
        T: Service<Request<B>, Response = Response<BoxBody>, Error = Infallible>
            + Clone
            + Send
            + 'static,
        T::Future: Send + 'static,
    {
        self.router = self.router.route(path, svc);
        self
    }

    pub fn index<H, T>(self, handler: H) -> Self
    where
        H: Handler<T, B>,
        T: 'static,
    {
        let path = self.index_create_path();
        self.route(&path, get(handler))
    }

    pub fn create<H, T>(self, handler: H) -> Self
    where
        H: Handler<T, B>,
        T: 'static,
    {
        let path = self.index_create_path();
        self.route(&path, post(handler))
    }

    pub fn new<H, T>(self, handler: H) -> Self
    where
        H: Handler<T, B>,
        T: 'static,
    {
        let path = format!("/{}/new", self.path);
        self.route(&path, get(handler))
    }

    pub fn show<H, T>(self, handler: H) -> Self
    where
        H: Handler<T, B>,
        T: 'static,
    {
        let path = self.show_update_destroy_path();
        self.route(&path, get(handler))
    }

    pub fn edit<H, T>(self, handler: H) -> Self
    where
        H: Handler<T, B>,
        T: 'static,
    {
        let path = format!("/{0}/:{0}_id/edit", self.path);
        self.route(&path, get(handler))
    }

    pub fn update<H, T>(self, handler: H) -> Self
    where
        H: Handler<T, B>,
        T: 'static,
    {
        let path = self.show_update_destroy_path();
        self.route(&path, on(MethodFilter::PUT | MethodFilter::PATCH, handler))
    }

    pub fn destroy<H, T>(self, handler: H) -> Self
    where
        H: Handler<T, B>,
        T: 'static,
    {
        let path = self.show_update_destroy_path();
        self.route(&path, delete(handler))
    }

    pub fn nest<T>(mut self, svc: T) -> Self
    where
        T: Service<Request<B>, Response = Response<BoxBody>, Error = Infallible>
            + Clone
            + Send
            + 'static,
        T::Future: Send + 'static,
    {
        let path = self.show_update_destroy_path();
        self.router = self.router.nest(&path, svc);
        self
    }

    pub fn nest_collection<T>(mut self, svc: T) -> Self
    where
        T: Service<Request<B>, Response = Response<BoxBody>, Error = Infallible>
            + Clone
            + Send
            + 'static,
        T::Future: Send + 'static,
    {
        let path = self.index_create_path();
        self.router = self.router.nest(&path, svc);
        self
    }
}

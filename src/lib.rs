#![allow(clippy::type_complexity)]

use axum::{
    body::{Body, BoxBody},
    handler::{self, Handler},
    http::{Request, Response},
    routing::{
        service_method_routing::{delete, get, on, post},
        MethodFilter,
    },
    Router,
};
use std::{
    convert::Infallible,
    marker::PhantomData,
    task::{Context, Poll},
};
use tower_service::Service;

pub struct Resource<Index, Create, New, Show, Edit, Update, Destroy, Nest, NestCollection, B = Body>
{
    path: String,
    index: Option<Index>,
    create: Option<Create>,
    new: Option<New>,
    show: Option<Show>,
    edit: Option<Edit>,
    update: Option<Update>,
    destroy: Option<Destroy>,
    nest: Option<Nest>,
    nest_collection: Option<NestCollection>,
    _body: PhantomData<fn() -> B>,
}

impl<B>
    Resource<
        NeverService,
        NeverService,
        NeverService,
        NeverService,
        NeverService,
        NeverService,
        NeverService,
        NeverService,
        NeverService,
        B,
    >
{
    pub fn named(path: &str) -> Self {
        Resource {
            path: path.to_string(),
            index: None,
            create: None,
            new: None,
            show: None,
            edit: None,
            update: None,
            destroy: None,
            nest: None,
            nest_collection: None,
            _body: PhantomData,
        }
    }
}

impl<Index, Create, New, Show, Edit, Update, Destroy, Nest, NestCollection, B>
    Resource<Index, Create, New, Show, Edit, Update, Destroy, Nest, NestCollection, B>
where
    B: Send + 'static,
{
    pub fn index<H, T>(
        self,
        handler: H,
    ) -> Resource<
        handler::IntoService<H, B, T>,
        Create,
        New,
        Show,
        Edit,
        Update,
        Destroy,
        Nest,
        NestCollection,
        B,
    >
    where
        H: Handler<B, T>,
    {
        Resource {
            path: self.path,
            index: Some(handler.into_service()),
            create: self.create,
            new: self.new,
            show: self.show,
            edit: self.edit,
            update: self.update,
            destroy: self.destroy,
            nest: self.nest,
            nest_collection: self.nest_collection,
            _body: PhantomData,
        }
    }

    pub fn create<H, T>(
        self,
        handler: H,
    ) -> Resource<
        Index,
        handler::IntoService<H, B, T>,
        New,
        Show,
        Edit,
        Update,
        Destroy,
        Nest,
        NestCollection,
        B,
    >
    where
        H: Handler<B, T>,
    {
        Resource {
            path: self.path,
            index: self.index,
            create: Some(handler.into_service()),
            new: self.new,
            show: self.show,
            edit: self.edit,
            update: self.update,
            destroy: self.destroy,
            nest: self.nest,
            nest_collection: self.nest_collection,
            _body: PhantomData,
        }
    }

    pub fn new<H, T>(
        self,
        handler: H,
    ) -> Resource<
        Index,
        Create,
        handler::IntoService<H, B, T>,
        Show,
        Edit,
        Update,
        Destroy,
        Nest,
        NestCollection,
        B,
    >
    where
        H: Handler<B, T>,
    {
        Resource {
            path: self.path,
            index: self.index,
            create: self.create,
            new: Some(handler.into_service()),
            show: self.show,
            edit: self.edit,
            update: self.update,
            destroy: self.destroy,
            nest: self.nest,
            nest_collection: self.nest_collection,
            _body: PhantomData,
        }
    }

    pub fn show<H, T>(
        self,
        handler: H,
    ) -> Resource<
        Index,
        Create,
        New,
        handler::IntoService<H, B, T>,
        Edit,
        Update,
        Destroy,
        Nest,
        NestCollection,
        B,
    >
    where
        H: Handler<B, T>,
    {
        Resource {
            path: self.path,
            index: self.index,
            create: self.create,
            new: self.new,
            show: Some(handler.into_service()),
            edit: self.edit,
            update: self.update,
            destroy: self.destroy,
            nest: self.nest,
            nest_collection: self.nest_collection,
            _body: PhantomData,
        }
    }

    pub fn edit<H, T>(
        self,
        handler: H,
    ) -> Resource<
        Index,
        Create,
        New,
        Show,
        handler::IntoService<H, B, T>,
        Update,
        Destroy,
        Nest,
        NestCollection,
        B,
    >
    where
        H: Handler<B, T>,
    {
        Resource {
            path: self.path,
            index: self.index,
            create: self.create,
            new: self.new,
            show: self.show,
            edit: Some(handler.into_service()),
            update: self.update,
            destroy: self.destroy,
            nest: self.nest,
            nest_collection: self.nest_collection,
            _body: PhantomData,
        }
    }

    pub fn update<H, T>(
        self,
        handler: H,
    ) -> Resource<
        Index,
        Create,
        New,
        Show,
        Edit,
        handler::IntoService<H, B, T>,
        Destroy,
        Nest,
        NestCollection,
        B,
    >
    where
        H: Handler<B, T>,
    {
        Resource {
            path: self.path,
            index: self.index,
            create: self.create,
            new: self.new,
            show: self.show,
            edit: self.edit,
            update: Some(handler.into_service()),
            destroy: self.destroy,
            nest: self.nest,
            nest_collection: self.nest_collection,
            _body: PhantomData,
        }
    }

    pub fn destroy<H, T>(
        self,
        handler: H,
    ) -> Resource<
        Index,
        Create,
        New,
        Show,
        Edit,
        Update,
        handler::IntoService<H, B, T>,
        Nest,
        NestCollection,
        B,
    >
    where
        H: Handler<B, T>,
    {
        Resource {
            path: self.path,
            index: self.index,
            create: self.create,
            new: self.new,
            show: self.show,
            edit: self.edit,
            update: self.update,
            destroy: Some(handler.into_service()),
            nest: self.nest,
            nest_collection: self.nest_collection,
            _body: PhantomData,
        }
    }

    pub fn nest<T>(
        self,
        svc: T,
    ) -> Resource<Index, Create, New, Show, Edit, Update, Destroy, T, NestCollection, B>
    where
        T: Service<Request<B>, Response = Response<BoxBody>, Error = Infallible>
            + Clone
            + Send
            + 'static,
        T::Future: Send + 'static,
    {
        Resource {
            path: self.path,
            index: self.index,
            create: self.create,
            new: self.new,
            show: self.show,
            edit: self.edit,
            update: self.update,
            destroy: self.destroy,
            nest: Some(svc),
            nest_collection: self.nest_collection,
            _body: PhantomData,
        }
    }

    pub fn nest_collection<T>(
        self,
        svc: T,
    ) -> Resource<Index, Create, New, Show, Edit, Update, Destroy, Nest, T, B>
    where
        T: Service<Request<B>, Response = Response<BoxBody>, Error = Infallible>
            + Clone
            + Send
            + 'static,
        T::Future: Send + 'static,
    {
        Resource {
            path: self.path,
            index: self.index,
            create: self.create,
            new: self.new,
            show: self.show,
            edit: self.edit,
            update: self.update,
            destroy: self.destroy,
            nest: self.nest,
            nest_collection: Some(svc),
            _body: PhantomData,
        }
    }

    pub fn into_router(self) -> Router<B>
    where
        Index: ResourceService<B>,
        Index::Future: Send + 'static,
        Create: ResourceService<B>,
        Create::Future: Send + 'static,
        New: ResourceService<B>,
        New::Future: Send + 'static,
        Show: ResourceService<B>,
        Show::Future: Send + 'static,
        Edit: ResourceService<B>,
        Edit::Future: Send + 'static,
        Update: ResourceService<B>,
        Update::Future: Send + 'static,
        Destroy: ResourceService<B>,
        Destroy::Future: Send + 'static,
        Nest: ResourceService<B>,
        Nest::Future: Send + 'static,
        NestCollection: ResourceService<B>,
        NestCollection::Future: Send + 'static,
    {
        let Self {
            path,
            index,
            create,
            new,
            show,
            edit,
            update,
            destroy,
            nest,
            nest_collection,
            _body: _,
        } = self;

        let mut router = Router::new();

        let index_create_path = format!("/{}", path);
        router = match (index, create) {
            (None, None) => router,
            (None, Some(create)) => router.route(&index_create_path, post(create)),
            (Some(index), None) => router.route(&index_create_path, get(index)),
            (Some(index), Some(create)) => {
                router.route(&index_create_path, get(index).post(create))
            }
        };

        if let Some(nest_collection) = nest_collection {
            router = router.nest(&index_create_path, nest_collection);
        }

        if let Some(new) = new {
            router = router.route(&format!("/{}/new", path), get(new))
        }

        if let Some(edit) = edit {
            router = router.route(&format!("/{}/:id/edit", path), get(edit))
        }

        let show_update_destroy_path = format!("/{}/:id", path);
        let put_or_patch = MethodFilter::PUT | MethodFilter::PATCH;
        router = match (show, update, destroy) {
            (None, None, None) => router,
            (None, None, Some(destroy)) => router.route(&show_update_destroy_path, delete(destroy)),
            (None, Some(update), None) => {
                router.route(&show_update_destroy_path, on(put_or_patch, update))
            }
            (None, Some(update), Some(destroy)) => router.route(
                &show_update_destroy_path,
                on(put_or_patch, update).delete(destroy),
            ),
            (Some(show), None, None) => router.route(&show_update_destroy_path, get(show)),
            (Some(show), None, Some(destroy)) => {
                router.route(&show_update_destroy_path, get(show).delete(destroy))
            }
            (Some(show), Some(update), None) => router.route(
                &show_update_destroy_path,
                get(show).on(put_or_patch, update),
            ),
            (Some(show), Some(update), Some(destroy)) => router.route(
                &show_update_destroy_path,
                get(show).on(put_or_patch, update).delete(destroy),
            ),
        };

        if let Some(nest) = nest {
            router = router.nest(&show_update_destroy_path, nest);
        }

        router
    }
}

#[derive(Debug, Copy, Clone)]
pub enum NeverService {}

impl<B> Service<Request<B>> for NeverService {
    type Response = Response<BoxBody>;
    type Error = Infallible;
    type Future = std::future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match *self {}
    }

    fn call(&mut self, _req: Request<B>) -> Self::Future {
        match *self {}
    }
}

pub trait ResourceService<B>:
    Service<Request<B>, Response = Response<BoxBody>, Error = Infallible>
    + Clone
    + Send
    + Sized
    + 'static
{
}

impl<B, T> ResourceService<B> for T where
    T: Service<Request<B>, Response = Response<BoxBody>, Error = Infallible>
        + Clone
        + Send
        + 'static
{
}

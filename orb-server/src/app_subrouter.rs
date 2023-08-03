use std::{convert::Infallible, task::Poll};

use axum::{http::Request, body::{HttpBody, Body}, response::Response, routing::future::RouteFuture, Router};
use tower::{Service, Layer};

use crate::RuntimeDb;

#[derive(Clone)]
pub struct Subrouter<S> {
    inner: S,
    runtime_db: RuntimeDb,
}

impl<S> Subrouter<S> {
    fn new(inner: S, runtime_db: RuntimeDb) -> Self {
        Self {
            inner,
            runtime_db,
        }
    }
}

async fn route_subdomain(subdomain: String) -> Result<(), Infallible> {
    todo!()
    // match subdomain {
    //     crate::config::HOSTNAME => self.inner.call_with_state(req, self.runtime_db.clone()),
    //     _ => {
    //         let runtime_db = rt.block_on((*self.runtime_db).lock());
    //         let orb = runtime_db.get(subdomain).unwrap();
    //         orb.router.call(req)
    //     },
    // }
}

impl<S, B> Service<Request<B>> for Subrouter<S>
    where S: Service<Request<B>>,
          S::Future: Into<RouteFuture<Response, Infallible>>,
{

    type Response = Response;

    type Error = Infallible;

    type Future = RouteFuture<Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        self.inner.call(req).into()
    }
}


#[derive(Clone)]
pub struct SubrouterLayer {
    runtime_db: RuntimeDb
}

impl SubrouterLayer {
    pub fn new(runtime_db: RuntimeDb) -> Self {
        Self { runtime_db }
    }
}

impl<S> Layer<S> for SubrouterLayer {
    type Service = Subrouter<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Subrouter::new(inner, self.runtime_db.clone())
    }
}

use super::{Context, MessageWriter, Params};
use crate::rpc;
use async_trait::async_trait;
use futures::{Future, SinkExt};
use lsp_types::{notification::Notification, request::Request};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

#[async_trait]
pub(crate) trait Handler<W: Clone + Send + Sync>: Send {
    fn method(&self) -> &'static str;

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        writer: Option<&mut dyn MessageWriter>,
    );

    fn box_clone(&self) -> Box<dyn Handler<W>>;
}

impl<W: Clone + Send + Sync> Clone for Box<dyn Handler<W>> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub struct RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>> + Send,
    W: Clone + Send + Sync,
{
    f: fn(Context<W>, Params<R::Params>) -> F,
    t: PhantomData<W>,
}

impl<R, F, W> Clone for RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>> + Send,
    W: Clone + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f,
            t: Default::default(),
        }
    }
}

impl<R, F, W> RequestHandler<R, F, W>
where
    R: Request,
    F: Future<Output = Result<R::Result, rpc::Error>> + Send,
    W: Clone + Send + Sync,
{
    pub fn new(f: fn(Context<W>, Params<R::Params>) -> F) -> Self {
        Self {
            f,
            t: Default::default(),
        }
    }
}

#[async_trait]
impl<R, F, P, W> Handler<W> for RequestHandler<R, F, W>
where
    R: Request<Params = P> + 'static,
    R::Result: Send,
    P: Send + Serialize + DeserializeOwned + 'static,
    F: Future<Output = Result<R::Result, rpc::Error>> + Send + 'static,
    W: Clone + Send + Sync + 'static,
{
    fn method(&self) -> &'static str {
        R::METHOD
    }

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        writer: Option<&mut dyn MessageWriter>,
    ) {
        let req_id = message.id.clone();
        let req = match message.into_params::<R::Params>() {
            Ok(r) => r,
            Err(e) => {
                if let Some(w) = writer {
                    w.send(
                        rpc::Response::error(rpc::Error::invalid_params().with_data(e.to_string()))
                            .with_request_id(req_id.unwrap())
                            .into_message(),
                    )
                    .await
                    .unwrap();
                }

                return;
            }
        };

        let call_result = (self.f)(context, req.params.into()).await;

        if let Some(w) = writer {
            let res = rpc::Response::from(call_result).with_request_id(req.id.unwrap());
            w.send(res.into_message()).await.unwrap();
        }
    }

    fn box_clone(&self) -> Box<dyn Handler<W>> {
        Box::new((*self).clone())
    }
}

#[derive(Clone)]
pub struct NotificationHandler<N, F, W>
where
    N: Notification,
    F: Future + Send,
    W: Clone + Send + Sync,
{
    f: fn(Context<W>, Params<N::Params>) -> F,
    t: PhantomData<W>,
}

impl<N, F, W> NotificationHandler<N, F, W>
where
    N: Notification,
    F: Future + Send,
    W: Clone + Send + Sync,
{
    pub fn new(f: fn(Context<W>, Params<N::Params>) -> F) -> Self {
        Self {
            f,
            t: Default::default(),
        }
    }
}

impl<N, F, W> NotificationHandler<N, F, W>
where
    N: Notification,
    F: Future + Send,
    W: Clone + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f,
            t: Default::default(),
        }
    }
}

#[async_trait]
impl<N, F, P, W> Handler<W> for NotificationHandler<N, F, W>
where
    N: Notification<Params = P> + 'static,
    P: Send + Serialize + DeserializeOwned + 'static,
    F: Future + Send + 'static,
    W: Clone + Send + Sync + 'static,
{
    fn method(&self) -> &'static str {
        N::METHOD
    }

    async fn handle(
        &mut self,
        context: Context<W>,
        message: rpc::Request<serde_json::Value>,
        _writer: Option<&mut dyn MessageWriter>,
    ) {
        let req = match message.into_params::<N::Params>() {
            Ok(r) => r,
            Err(_) => return,
        };

        (self.f)(context, req.params.into()).await;
    }

    fn box_clone(&self) -> Box<dyn Handler<W>> {
        Box::new((*self).clone())
    }
}

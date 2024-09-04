use dioxus::prelude::*;

pub mod session;

pub struct Backend {
    pub foo: i32,
}

impl FromServerContext for Backend {
    type Rejection = BackendLayerNotFound;

    fn from_request<'life0, 'async_trait>(
        _: &'life0 DioxusServerContext,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<Self, Self::Rejection>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }
}

#[derive(Debug)]
pub struct BackendLayerNotFound;

impl std::fmt::Display for BackendLayerNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Backend was not found")
    }
}

impl std::error::Error for BackendLayerNotFound {}

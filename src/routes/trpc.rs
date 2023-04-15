pub fn rspc_router() -> axum::Router {
    rspc::Router::<()>::new()
        .query("version", |t| {
            t(|_ctx, _input: ()| env!("CARGO_PKG_VERSION"))
        })
        .build()
        .arced()
        .endpoint(|| ()).axum()
}

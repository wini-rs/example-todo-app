use {
    crate::{
        pages,
        shared::wini::{layer::MetaLayerBuilder, PORT},
        template,
        utils::wini::{
            cache,
            handling_file::{self},
        },
    },
    axum::{
        middleware,
        routing::{delete, get, post, put},
        Router,
    },
    log::info,
    std::collections::HashMap,
    tower_http::compression::CompressionLayer,
};


pub async fn start() {
    // The main router of the application is defined here
    let app = Router::<()>::new()
        .route("/", get(pages::todo::render))
        .layer(
            MetaLayerBuilder::default()
                .default_meta(HashMap::from_iter([
                    ("title", "Wini todo".into()),
                    ("description", "wini todo example".into()),
                ]))
                .build()
                .expect("Failed to build MetaLayer"),
        )
        .layer(middleware::from_fn(template::template))
        .layer(middleware::from_fn(cache::html_middleware))
        .route("/{*wildcard}", get(handling_file::handle_file))
        .route("/task/{id}/done", put(pages::todo::db::done))
        .route(
            "/task/{id}",
            delete(pages::todo::db::delete).put(pages::todo::db::edit_name),
        )
        .route("/task", post(pages::todo::db::create))
        .layer(CompressionLayer::new());


    // Start the server
    info!("Starting listening on port {}...", *PORT);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", *PORT))
        .await
        .expect("Couldn't start the TcpListener of the specified port.");

    info!("Starting the server...");
    axum::serve(listener, app)
        .await
        .expect("Couldn't start the server.");
}

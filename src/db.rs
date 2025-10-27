use {
    sqlx::{postgres::PgPoolOptions, PgPool},
    std::sync::LazyLock,
    tokio::{runtime::Handle, task::block_in_place},
};

pub static DB: LazyLock<PgPool> = LazyLock::new(|| {
    block_in_place(move || {
        Handle::current().block_on(async move {
            PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://postgres:@localhost:5432/todo")
                .await
                .expect("Couldn't connect to database")
        })
    })
});

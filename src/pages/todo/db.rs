use {
    crate::{db::DB, pages::todo::render_task, shared::wini::err::ServerResult},
    axum::{extract::Path, response::IntoResponse, Form},
    hyper::StatusCode,
    maud::html,
};

pub async fn delete(Path(id): Path<i32>) -> ServerResult<()> {
    sqlx::query!("delete from tasks where id = $1", id)
        .execute(&*DB)
        .await?;

    Ok(())
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Create {
    title: String,
}
pub async fn create(Form(Create { title }): Form<Create>) -> ServerResult<impl IntoResponse> {
    let task = sqlx::query_as!(
        super::Task,
        "insert into tasks(title)
        values ($1)
        returning id, title, is_done",
        title
    )
    .fetch_one(&*DB)
    .await?;

    Ok(html! {
        li {
            [render_task(task)]
        }
    })
}

pub async fn done(Path(id): Path<i32>) -> ServerResult<StatusCode> {
    sqlx::query!("update tasks set is_done = not is_done where id = $1", id)
        .execute(&*DB)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn edit_name(
    Path(id): Path<i32>,
    Form(Create { title }): Form<Create>,
) -> ServerResult<impl IntoResponse> {
    let task = sqlx::query_as!(
        super::Task,
        "update tasks set title = $2 where id = $1 returning id, title, is_done",
        id,
        title
    )
    .fetch_one(&*DB)
    .await?;

    Ok(render_task(task).await)
}

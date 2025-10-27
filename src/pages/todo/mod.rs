pub mod db;

use {
    crate::{db::DB, shared::wini::err::ServerResult},
    maud::{html, Markup},
    wini_macros::{component, page},
};

struct Task {
    id: i32,
    is_done: bool,
    title: String,
}

#[page(js_pkgs=["htmx.org", "alpinejs"])]
pub async fn render() -> ServerResult<Markup> {
    let tasks = sqlx::query_as!(Task, "select * from tasks order by id")
        .fetch_all(&*DB)
        .await?;

    Ok(html! {
        main .todo {
            h1 {
                "TODO!"
            }
            ul {
                @for task in tasks {
                    li {
                        [render_task(task)]
                    }
                }
            }
            #create-bar {
                input id="title" name="title" type="text";
                button hx-post="/task" hx-include="previous input" hx-target="previous ul" hx-swap="beforeend" {
                    "Create"
                }
            }
        }
    })
}

#[component(js_pkgs=["htmx.org", "alpinejs"])]
async fn render_task(task: Task) -> Markup {
    html! {
        .task x-data={"{isEditting:false,isDone:"(task.is_done)"}"} x-bind:class="isDone && 'done'" {
            div {
                button
                    x-text="isDone ? 'Mark as todo' : 'Mark as done'"
                    x-on:click="isDone = !isDone"
                    hx-put={"/task/"(task.id)"/done"}
                    {
                        (if task.is_done {"Mark as todo"} else {"Mark as done"})
                    }
                span x-show="!isEditting" {
                    (task.title)
                }
                input x-show="isEditting" value=(task.title) name="title";
            }
            div {
                button
                    hx-put={"/task/"(task.id)""}
                    hx-target="closest .task"
                    hx-include="previous input"
                    hx-swap="outerHTML"
                    x-show="isEditting"
                    {
                        "Save"
                    }
                button
                    x-on:click="isEditting = !isEditting"
                    x-text="isEditting ? 'Cancel' : 'Edit'"
                    {}
                button
                    hx-delete={"/task/"(task.id)}
                    hx-target="closest .task"
                    x-show="!isEditting"
                    { "Delete" }
            }
        }
    }
}

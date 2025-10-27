pub mod db;

use {
    crate::{db::DB, shared::wini::err::ServerResult},
    font_awesome_as_a_crate::{svg, Type},
    maud::{html, Markup, PreEscaped},
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
                "TODO: wini + htmx + alpine"
            }
            ul {
                @for task in tasks {
                    li {
                        [render_task(task)]
                    }
                }
            }
            #create-bar {
                input name="title" type="text" placeholder="Create a new task...";
                button.green
                    hx-post="/task"
                    hx-include="previous input"
                    hx-target="previous ul"
                    hx-swap="beforeend"
                    { (icon("square-plus")) }
            }
        }
    })
}

#[component(js_pkgs=["htmx.org", "alpinejs"])]
async fn render_task(task: Task) -> Markup {
    html! {
        .task x-data={"{isEditing:false,isDone:"(task.is_done)"}"} x-bind:class="isDone && 'done'" {
            div {
                input
                    type="checkbox"
                    x-bind:checked="isDone ? true : false"
                    x-on:click="isDone = !isDone"
                    hx-put={"/task/"(task.id)"/done"};
                span x-show="!isEditing" {
                    (task.title)
                }
                input x-show="isEditing" value=(task.title) name="title" type="text";
            }
            div {
                // When editing
                button.green
                    hx-put={"/task/"(task.id)""}
                    hx-target="closest .task"
                    hx-include="previous input"
                    hx-swap="outerHTML"
                    x-show="isEditing"
                    { (icon("check")) }
                button.red
                    x-on:click="isEditing = !isEditing"
                    x-show="isEditing"
                    { (icon("rotate-left")) }

                // When not editing
                button.blue
                    x-on:click="isEditing = !isEditing"
                    x-show="!isEditing"
                    { (icon("pen")) }
                button.red
                    hx-delete={"/task/"(task.id)}
                    hx-target="closest li"
                    hx-swap="outerHTML"
                    x-show="!isEditing"
                    { (icon("trash")) }
            }
        }
    }
}

fn icon(name: &'static str) -> PreEscaped<&'static str> {
    PreEscaped(svg(Type::Solid, name).unwrap())
}

create table tasks (
    id integer primary key generated always as identity,
    title text not null,
    is_done bool not null default false
);

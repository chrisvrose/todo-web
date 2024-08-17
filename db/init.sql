create table users(
    id serial primary key,
    email varchar(64) unique not null
);

create table projects (
    id serial primary key,
    title varchar(32) not null,
    description text,
    date_added timestamp with time zone not null,
    date_modified timestamp with time zone not null
);

create table todo (
    id serial primary key,
    owner integer references users(id),
    title varchar(64) not null,
    state char(4) not null,
    description text,
    project_grouping integer references projects(id),
    date_added timestamp with time zone not null,
    date_updated timestamp with time zone not null
);

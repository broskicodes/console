create type prompt_flavour as enum (
  'initial_goals'
);

create table chats (
  id uuid primary key default gen_random_uuid (),
  flavour prompt_flavour not null,
  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone,
  deleted_at timestamp with time zone
);

create table messages (
  id uuid primary key default gen_random_uuid (),
  role text not null,
  content text not null,
  chat_id uuid not null references chats (id),
  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone,
  deleted_at timestamp with time zone
);
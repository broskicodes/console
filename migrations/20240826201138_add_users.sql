create table users (
  id uuid primary key default gen_random_uuid (),
  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone,
  deleted_at timestamp with time zone
);

alter table chats
add column user_id uuid not null references users (id);
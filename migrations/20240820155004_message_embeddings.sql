create table message_embeddings (
  id uuid primary key default gen_random_uuid (),
  message_id uuid not null,
  embedding vector (384) not null,
  section smallint,
  foreign key (message_id) references messages (id)
);
create type chat_prompt as enum (
  'initial_goals',
  'daily_outline'
);

ALTER TABLE chats
ALTER COLUMN flavour TYPE chat_prompt USING flavour::text::chat_prompt;

DROP TYPE prompt_flavour;
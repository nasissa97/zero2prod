-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES(
  'ddf8994f-d522-4659-8d02-c1d479057be6',
  'admin',
  '$argon2id$v=19$m=15000,t=2,p=1$fPxp5L1xoX98yWmjmeVZbw$2/tUPv5x4RfLWLRSb+Gs0d02P+yNPR2rCJQOuUkR528'
);
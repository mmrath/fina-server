CREATE TABLE message (
  id           BIGSERIAL PRIMARY KEY,
  created_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  subject      TEXT                     NOT NULL,
  message_type TEXT                     NOT NULL,
  body_type    TEXT                     NOT NULL,
  body         TEXT                     NOT NULL
);

CREATE TABLE message_address (
  id           BIGSERIAL PRIMARY KEY,
  message_id   BIGINT NOT NULL,
  address_type TEXT   NOT NULL,
  name         TEXT,
  address      TEXT,
  CONSTRAINT fk_message_id FOREIGN KEY (message_id) REFERENCES message (id)
);


CREATE TABLE message_attachment (
  id         BIGSERIAL PRIMARY KEY,
  message_id BIGINT NOT NULL,
  name       TEXT,
  DATA       BYTEA,
  CONSTRAINT fk_message_id FOREIGN KEY (message_id) REFERENCES message (id)

);

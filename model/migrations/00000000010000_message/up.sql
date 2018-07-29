CREATE TABLE message (
  id           BIGSERIAL,
  created_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
  subject      TEXT                     NOT NULL,
  message_type TEXT                     NOT NULL,
  body_type    TEXT                     NOT NULL,
  body         TEXT                     NOT NULL,
  CONSTRAINT pk_message PRIMARY KEY (id)
);

CREATE TABLE message_address (
  id           BIGSERIAL ,
  message_id   BIGINT NOT NULL,
  address_type TEXT   NOT NULL,
  name         TEXT,
  address      TEXT NOT NULL,
  CONSTRAINT pk_message_address PRIMARY KEY (id),
  CONSTRAINT fk_message_address__message FOREIGN KEY (message_id) REFERENCES message (id)
);


CREATE TABLE message_attachment (
  id         BIGSERIAL,
  message_id BIGINT NOT NULL,
  name       TEXT NOT NULL,
  DATA       BYTEA NOT NULL,
  CONSTRAINT pk_message_attachment PRIMARY KEY (id),
  CONSTRAINT fk_message_attachment__message FOREIGN KEY (message_id) REFERENCES message (id)
);

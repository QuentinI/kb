CREATE TABLE tickets (
       id              char(32)   PRIMARY KEY,
       departure_code  char(3)    NOT NULL,
       arrival_code    char(3)    NOT NULL,
       departure_time  timestamp  NOT NULL,
       arrival_time    timestamp  NOT NULL,
       price           integer    NOT NULL
);

CREATE TYPE tickets_batch_pretransform AS (
       id              char(32),
       departure_code  char(3),
       arrival_code    char(3),
       departure_time  text,
       arrival_time    text,
       price           integer
);

CREATE INDEX tickets_departure_code_index ON tickets USING HASH (departure_code);
CREATE INDEX tickets_arrival_code_index   ON tickets USING HASH (arrival_code);
CREATE INDEX tickets_departure_time_index ON tickets (departure_time);
CREATE INDEX tickets_arrival_time_index   ON tickets (arrival_time);
CREATE INDEX tickets_price_index          ON tickets (price);

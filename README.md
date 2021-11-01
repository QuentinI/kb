# Test for Kupibilet

## Setup:

Prerequisites: rust, docker, [migrant](https://github.com/jaemk/migrant)

``` shellsession
$ docker-compose --env-file .env up -d # start Postgres
$ migrant setup                        # apply migrations
$ migrant apply
$ cargo run
```

Alternatively, manually applying migrations after starting the database:
``` shellsession
$ psql -h localhost -U postgres postgres -f ./migrations/20211023145913_create-tickets/up.sql
```

## Usage
As per task requirements.

API is available on port 8000. Some sample data is available in associated repository [kupibilet_data](https://github.com/QuentinI/kupibilet_data) folder, taken from [Kaggle](https://www.kaggle.com/tylerx/flights-and-airports-data?select=flights.csv).

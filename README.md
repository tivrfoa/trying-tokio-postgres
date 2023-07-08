

```sh
docker run --name tokio1 -e POSTGRES_PASSWORD=123 -d -p 5432:5432 postgres:9.6.24

docker exec -it tokio1 /bin/bash

root@380f8cd4dba6:/# psql -U postgres
psql (9.6.24)
Type "help" for help.

postgres=# CREATE DATABASE mytest;
postgres=# \c mytest
```

```sql
SELECT current_database();


CREATE TABLE address (
    id SERIAL PRIMARY KEY,
    street VARCHAR(255)
);

insert into address (street) values ('PV 401');

select * from address;
```

## References

https://stackoverflow.com/questions/37694987/connecting-to-postgresql-in-a-docker-container-from-outside

https://stackoverflow.com/questions/70021392/rust-and-postgresql-with-tokio-postgres

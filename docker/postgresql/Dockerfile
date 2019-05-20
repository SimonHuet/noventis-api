FROM postgres:9.5-alpine

RUN  /etc/init.d/postgresql start &&\
    psql --command "CREATE USER noventis WITH SUPERUSER PASSWORD 'noventis';" &&\
    createdb -O noventis noventis

FROM postgres:17-alpine

WORKDIR /docker-entrypoint-initdb.d/
COPY init.sql /docker-entrypoint-initdb.d/
RUN chmod 0755 /docker-entrypoint-initdb.d/init.sql
EXPOSE 5432
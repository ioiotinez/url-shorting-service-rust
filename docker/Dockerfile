FROM mysql:latest

ENV MYSQL_ROOT_PASSWORD=root_password
ENV MYSQL_DATABASE=url_shortening_service

COPY init.sql /docker-entrypoint-initdb.d/

CMD ["mysqld"]
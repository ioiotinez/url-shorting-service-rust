FROM mysql:latest

ENV MYSQL_ROOT_PASSWORD=root_password
ENV MYSQL_DATABASE=url_shortening_service

COPY init.sql /docker-entrypoint-initdb.d/

#Execute the SQL script to create the database and tables
RUN ["mysql", "-u", "root", "-p$MYSQL_ROOT_PASSWORD", "$MYSQL_DATABASE", "<", "/docker-entrypoint-initdb.d/init.sql"]

CMD ["mysqld"]
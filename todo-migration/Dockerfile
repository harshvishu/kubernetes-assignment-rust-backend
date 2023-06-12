FROM postgres:latest

# Copy the migration SQL file into the image
COPY migration.sql /docker-entrypoint-initdb.d/

# Grant permissions to the SQL file
RUN chown postgres:postgres /docker-entrypoint-initdb.d/migration.sql
RUN chmod 755 /docker-entrypoint-initdb.d/migration.sql

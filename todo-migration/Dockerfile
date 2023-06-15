FROM postgres:latest

# Copy the migration SQL file into the image
COPY up.sql /docker-entrypoint-initdb.d/
COPY down.sql /docker-entrypoint-initdb.d/
COPY seed.sql /docker-entrypoint-initdb.d/

# Grant permissions to the up SQL file
RUN chown postgres:postgres /docker-entrypoint-initdb.d/up.sql
RUN chmod 755 /docker-entrypoint-initdb.d/up.sql
# Grant permissions to the down SQL file
RUN chown postgres:postgres /docker-entrypoint-initdb.d/down.sql
RUN chmod 755 /docker-entrypoint-initdb.d/down.sql
# Grant permissions to the seed SQL file
RUN chown postgres:postgres /docker-entrypoint-initdb.d/seed.sql
RUN chmod 755 /docker-entrypoint-initdb.d/seed.sql
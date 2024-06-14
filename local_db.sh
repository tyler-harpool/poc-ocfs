!#/bin/bash
podman run --name my_postgres -e POSTGRES_USER=app_user -e POSTGRES_PASSWORD=dev_only_pwd -e POSTGRES_DB=app_db -p 5432:5432 -d postgres:latest

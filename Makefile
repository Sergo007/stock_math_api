IMAGE_LOCATION=gcr.io/ecare21-dev
IMAGE_NAME=stock_math_api
IMAGE_TAG=0.2.4

docker-build:
	docker build --target runtime -t ${IMAGE_LOCATION}/${IMAGE_NAME}:${IMAGE_TAG} -f Dockerfile .

docker-build-debian:
	docker build -t ${IMAGE_LOCATION}/${IMAGE_NAME}:${IMAGE_TAG} -f debian/Dockerfile .

docker-push:
	docker push ${IMAGE_LOCATION}/${IMAGE_NAME}:${IMAGE_TAG}

docker-run:
	docker run -p 8080:8080 ${IMAGE_LOCATION}/${IMAGE_NAME}:${IMAGE_TAG}

sql_gen_generate:
	sql-gen generate --output src/db --schema pcc_athena_interop --force  --database postgresql://admin:`cat ./credentials/db_password_admin`@localhost:5444/pcc_athena_interop_dev

sql_gen_migrate_generate:
	sql-gen migrate --include src/db --output migrations --schema pcc_athena_interop --database postgresql://admin:`cat ./credentials/db_password_admin`@localhost:5444/pcc_athena_interop_dev 

sql_gen_install_local:
	cargo install --path /Users/yatsinaserhii/projects/cybx/sre-university/rust_examples/sql-gen

sql_gen_install:
	cargo install --git https://github.com/Sergo007/sql-gen --branch fix_bugs

watch:
	cargo watch -x 'run'
MAKEFLAGS=--no-builtin-rules --no-builtin-variables --always-make
ROOT := $(realpath $(dir $(lastword $(MAKEFILE_LIST))))

clean:
	cargo clean

build:
	cd api && cargo build

deploy:
	gcloud config set project rust-gae-cloudsql-sample
	gcloud config set app/cloud_build_timeout 1200
	gcloud app deploy

run:
	cd api && \
	DATABASE_URL=mysql://user:password@127.0.0.1:3306/db \
	ADDRESS=127.0.0.1 cargo run

run-linux:
	docker build -t rust-gae-cloudsql-sample-local .
	docker run --rm -p 8080:8080 rust-gae-cloudsql-sample-local

run-db:
	docker-compose up
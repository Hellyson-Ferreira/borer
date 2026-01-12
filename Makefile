IMAGE_NAME := hellysonf/borer-server
TAG ?= latest

# -----------------------
# Docker
# -----------------------

build:
	docker compose build

push:
	docker push $(IMAGE_NAME):$(TAG)

build-push: build push

run:
	docker compose up -d

stop:
	docker compose down

# -----------------------
# Cargo (local dev)
# -----------------------

borer-server:
	cargo run --bin borerd

borer-client-up:
	cargo run --bin borer -- up 3001

borer-client-logout:
	cargo run --bin borer -- logout

# -----------------------
# Utils
# -----------------------

run-migrations:
	cd crates/borer-server && set -a && source .env && sqlx migrate run

docker-migrations:
	docker compose up migrations --build

logs:
	docker compose logs -f

restart: stop run

.PHONY: \
	build push build-push run stop restart logs \
	borer-server borer-client-up borer-client-logout \
	run-migrations docker-migrations

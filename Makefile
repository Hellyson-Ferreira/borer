IMAGE_NAME = hellysonf/borer-server
TAG ?= latest

build:
	docker compose build

push:
	docker push $(IMAGE_NAME):$(TAG)

build-push: build push

run:
	docker compose up -d

stop:
	docker compose down

.PHONY: build push build-push run stop


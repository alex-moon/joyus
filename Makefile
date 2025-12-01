SHELL := /bin/sh

.PHONY: dev build run

dev:
	npm run dev

build:
	docker-compose -f docker-compose.local.yml build --progress plain

run:
	docker compose --progress plain -f docker-compose.local.yml up

sh:
	docker-compose -f docker-compose.local.yml run --rm app bash

db:
	docker-compose -f docker-compose.local.yml run --rm \
	-e PGPASSWORD=joyus app psql --user joyus --host db joyus

deploy:
	./deploy.sh

SHELL := /bin/sh

.PHONY: dev build run

dev:
	npm run dev

build:
	docker-compose -f docker-compose.yml build --progress plain

run:
	docker compose --progress plain -f docker-compose.local.yml up

SHELL := /bin/sh

.PHONY: dev build run

dev:
	npm run dev

build.local:
	docker-compose -f docker-compose.local.yml build --progress plain

build:
	docker-compose -f docker-compose.yml build --progress plain

up:
	docker compose -f docker-compose.local.yml up

run: build.local up

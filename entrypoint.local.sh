#!/usr/bin/env bash
set -e

until pg_isready -h db -U joyus -d joyus; do
  sleep 1
done

sqlx migrate run

npm run dev

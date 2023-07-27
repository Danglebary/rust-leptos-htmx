db_create:
	sqlx database create --database-url postgres://postgres@localhost:5432/tester

db_create_migration:
	sqlx migrate add -r $(ARGS)

db_migrate:
	sqlx migrate run

db_start_local:
	docker-compose up -d --build

db_end_local:
	docker-compose down --rmi 'all' --remove-orphans -v
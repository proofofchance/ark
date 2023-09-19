db.start:
	docker-compose up

db.setup:
	diesel setup && diesel migration run

db.stop: 
	docker-compose down

db.drop:
	rm -rf ./postgres-data

db.reset:
	make db.stop && make db.drop && make db.start

setup: 
	cd ark-db && diesel setup && cd ..

web.dev:
	RUST_BACKTRACE=1 cargo watch -x 'run --bin ark-web'

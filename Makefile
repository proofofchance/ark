db.start:
	docker-compose up

db.setup: 
	cd ark-db && diesel setup && cd ..

db.stop: 
	docker-compose down

db.drop:
	rm -rf ./postgres-data

db.reset:
	make db.stop && make db.drop && make db.start

web.dev:
	RUST_BACKTRACE=1 cargo watch -x 'run --bin ark-web'

web.dev.reset:
	npx kill-port 4446 && make db.setup && make web.dev

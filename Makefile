db.start:
	docker-compose up

setup: 
	cd ark-db && diesel setup && cd ..

web.dev:
	cargo watch -x 'run --bin ark-web'

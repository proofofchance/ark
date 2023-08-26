up:
	docker-compose up

setup: 
	cd ark-db && diesel setup && cd ..

dev.web:
	cargo watch -x 'run --bin coinflip-web'

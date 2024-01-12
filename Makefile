db.start:
	docker-compose up

db.setup: 
	cd ark-db && diesel setup && cd .. 

db.setup.after_sleep:
	sleep 20 && make db.setup &

db.stop: 
	docker-compose down

db.drop:
	rm -rf ./postgres-data

db.reset:
	make db.stop && make db.drop && make db.start &

web.dev:
	RUST_BACKTRACE=1 cargo watch -x 'run --bin ark-web'

web.dev.reset:
	npx kill-port 4446 && make db.setup && make web.dev

web.dev.reset.after_sleep:
	sleep 40 && npx kill-port 4446 && cargo run ark-web &

reset:
	make db.reset && make db.setup.after_sleep && make web.dev.reset.after_sleep
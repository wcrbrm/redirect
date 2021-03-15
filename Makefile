build:
	cargo build --release
docker:
	docker build -t wcrbrm/redirect .
	docker run -d -e REDIRECT=http://localhost -p3000:3000 wcrbrm/redirect

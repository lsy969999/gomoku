clean:
	cargo clean

build-ws:
	cargo lambda build -p gomoku-ws --release --target=x86_64-unknown-linux-musl
	# upx --best target/lambda/gomoku-ws/bootstrap 
deploy-ws-dev:
	cargo lambda deploy --include Settings.toml:./gomoku-ws/Settings.toml --binary-name gomoku-ws gomoku-ws-dev

build-api:
	cargo lambda build -p gomoku-api --release --target=x86_64-unknown-linux-musl
	# upx --best target/lambda/gomoku-api/bootstrap 
deploy-api-dev:
	cargo lambda deploy --include Settings.toml:./gomoku-api/Settings.toml --binary-name gomoku-api gomoku-api-dev
	
build-q:
	cargo lambda build -p gomoku-q --release --target=x86_64-unknown-linux-musl
deploy-q-dev:
	cargo lambda deploy --include Settings.toml:./gomoku-q/Settings.toml --binary-name gomoku-q gomoku-q-dev
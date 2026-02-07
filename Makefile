deploy:
	cargo build --release && sudo systemctl stop fitness && sudo cp ./target/release/fitness-server /opt/fitness/ && sudo systemctl start fitness

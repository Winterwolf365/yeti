# use case
This reposetory is to test out how it is, to make an api in rust with rocket and diesel.

# instalation
Make sure you have following installed:
1. rust and cargo (https://rustup.rs/).
2. docker with compose (https://docs.docker.com/engine/install/).

Than you need to go into the direcotry of this reposetory and run the folling commands:
1. `sudo docker compose up -d` (in some cases you need to use docker-compose with and '-' between it, and the -d flag is to run it in the background)
2. `cargo run --release`
The docker command was to start the postgres database, and the cargo to run the rust project.

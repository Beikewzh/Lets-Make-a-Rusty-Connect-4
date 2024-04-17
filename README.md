### ECE 421 Group Project 3
Let's Make a Rusty Connect-4

## Quick Start

```sh
Firstly, we open two terminals. 
One for Server.
One for client. 
```

# Server 
1. Update dependencies as recorded in the local lock file
```sh
cargo update
```

2. Update rust toolchain installer
```sh
rustup update
```

3. Use Rust Nightly
```sh
rustup override set nightly
```

4. Download MongoDB (Followed the MongoDB offical website)
```sh
https://www.mongodb.com/docs/manual/administration/install-community/
```

5. Set up Server and open the MongoDB shell
```sh
mongod
mongo
```

6. Initialized the Database
If we already have ServerDB, then
```sh
use ServerDB
db.dropDatabase() 
```
else
```sh
use ServerDB
db.createCollection("users")
db.users.createIndex({"username": 1}, {unique: true})
db.createCollection("scores")
db.scores.createIndex({"username": 1}, {unique: true})
```

7. Run the Server
```sh
cargo run
```

# Client
1. Install Trunk (a WASM web application bundler for Rust)
```sh
cargo install trunk
```

1. Install the wasm-bindgen Command Line Interface
```sh
cargo install wasm-bindgen-cli
```

3. Run the client with specific port
```sh
trunk serve --port XXXX
```

4. Go the the website and start application
```sh
localhost:XXXX
```

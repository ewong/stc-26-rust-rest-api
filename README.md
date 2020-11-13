# stc-26-rust-rest-api
REST API example for Rust and Actix-web

1. Example project setup.
a. cd into project root directory directory (i.e. stc-26-rust-rest-api)
b. run: cargo install
c. run: cargo install diesel_cli => installs diesel command line interface
d. cd into db directory (i.e. stc-26-rust-rest-api/src/db)
e. run: diesel setup => creates the stc_26_rust_rest_api database
f. run: diesel migration run => sets up posts table 

2. Running example project.
a. cd into project root directory (stc-26-rust-rest-api)
b. run: cargo run => it will run the server on port 8080 (i.e. 127.0.0.1:8080)

3. Testing example project.
a. create: 
curl -d '{"title": "Hello World", "body": "This article shows you how to wave hello to the world!"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/post
curl -d '{"title": "Hello Saturn", "body": "This article shows you how to wave hello to Saturn!"}' -H 'Content-Type: application/json' http://127.0.0.1:8080/post
b. read all: 
curl -G -H 'Content-Type: application/json' http://127.0.0.1:8080/post
c. read one: 
curl -G -H 'Content-Type: application/json' http://127.0.0.1:8080/post/hello-world
curl -G -H 'Content-Type: application/json' http://127.0.0.1:8080/post/hello-saturn
d. update: 
curl -d '{"title": "Hello World blah", "body": "This article shows you how to wave hello to the world blah!", "published": true}' -H 'Content-Type: application/json' -X PUT http://127.0.0.1:8080/post/hello-world
e. delete: 
curl -H 'Content-Type: application/json' -X DELETE http://127.0.0.1:8080/post/hello-world-blah

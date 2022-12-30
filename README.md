# Maths

This is the library for the maths and (future) AI of the Prophecy Epitech project.
It can be used in different ways :
- as a dynamic library
- as a web server to call the functions with a HTTP request (the API is [here](https://app.swaggerhub.com/apis/Pepiloto/Prophecy-maths/1.0.0))

# If you want to contribute you can join our [Discord](https://discord.gg/2bCeWjzV) server !

Also read how to contribute [here](CONTRIBUTING.md).

# How to build it

## Dynamic library

```
cargo build --lib
```
or
```
# You will need docker for this
chmod +x build_lib.sh
./build_lib.sh
```
You can add `--release` to build it in release mode if you use `cargo build`.

## Web server

```
cargo build --bin prophecy-maths
```
or
```
# You will need docker for this
chmod +x build_server.sh
./build_server.sh
```
You can add `--release` to build it in release mode if you use `cargo build`.

# How to use it

## Web server

```
./prophecy-maths
```
or
```
cargo run
```
You can add `--release` to run it in release mode if you use `cargo run`.

# What to send to the web server

## Request

The request must be a JSON object sent with a POST request on /units.
An example is given in the file [request.example](request.example).

## Response

The response is a JSON object.
An example is given in the file [response.example](response.example).

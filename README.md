# Introduction

This is a project generated by [salvo-cli](https://github.com/salvo-rs/salvo-cli). You can run the program and test
according to the following commands (for non-sqlite databases, please modify the database connection string first
according to the tutorial, and complete the initial work of the data).
😄 The latest version of Salvo requires Rust version 1.75. If your compilation fails, please try upgrading
with `rustup update`.

## run in local

``` shell
//Run the project
cargo run 
//Run tests
cargo test
```

# Tip

- If you choose sqlite or have initialized the data in the users table, please use the account zhangsan with password
  123 to login.

# orm doc or home page link

You chose rbatis, please check the documentation here https://rbatis.github.io/rbatis.io/#/v4/

## Data initialization

- Please execute the sql files under the data folder to initialize the data before running

---

# salvo_demo_rbatis

## publish

### build by docker

```shell
docker build -t salvo_demo_rbatis .
#
docker images
```

### build by docker-compose

```shell
docker compose up --build
```

## run

## run by docker-compose

```shell
docker compose up -d --force-recreate
```

---

# create a Dockerfile for it

https://docs.docker.com/language/rust/build-images/

```shell
docker init
```

---

# About Salvo

You can view the salvo documentation and more examples at https://salvo.rs/ 📖. If our tools have helped you, please
star [salvo](https://github.com/salvo-rs/salvo) and [salvo-cli](https://github.com/salvo-rs/salvo-cli), which will
greatly encourage us. ❤️

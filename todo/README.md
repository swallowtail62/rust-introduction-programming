# TODO web application

This is a ToDo web application using [actix-web](https://github.com/actix/actix-web) crate.

## Feature

You can do the following

- Create a todo
- Read todo (see all)
- Delete todo

## To launch the app

There are two ways to run the app. One is to use docker container. The other is to run it locally.

In either case, execute the following command in the root direcotry.

### Docker container

```shell
$ docker build -t ${some_tag} .
$ docker run -p 8080:8080 ${some_tag}
```

### Local

- debug mode
```shell
$ cargo run
```

- production mode
```shell
$ cargo build --release
$ target/release/todo
```
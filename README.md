# bot server

### hot reloading

- [document]<https://github.com/watchexec/cargo-watch>

1. install

```
cargo install cargo-watch
```

2. run

```
cargo watch -x 'run algo-project-server'
```

### Docker

1. download docker rust

```
docker pull rust
```

2. build

```
docker build -t algo-project-server .
```

3. run

```
docker run --rm -dp 8080:8080  algo-project-server
```

### Docker Compose

run

```
docker-compose up -d
```

### Docker mysql

1. bash

```
docker exec -i -t algo-project-mysql bash
```

2. login in mysql bash

```
mysql -u root -p
```

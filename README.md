# rust-docker

## Setup

```
git clone git@github.com:You-saku/rust-dokcer.git
cd rust-docker
docker-compose up -d --build
docker-compose exec rust sh 
cd hello
cargo run
```

## 作業するときは

```
docker-compose exec rust sh 
```
でコンテナに入ってからやると良いです

## rustのドキュメント
https://doc.rust-jp.rs/book-ja/title-page.html
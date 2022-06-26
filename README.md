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

## Setup(web-server)
```
git clone git@github.com:You-saku/rust-dokcer.git
cd rust-docker
docker-compose up -d --build
docker-compose exec rust sh 
cd web
cargo run
http://0.0.0.0:7878
```

## 作業するときは

```
docker-compose exec rust sh 
```
でコンテナに入ってからやると良いです

## お役立ち情報
* cargo new {プロジェクト名} をしたらプロジェクトのルートに```.gitignore```を作って```/target```を書いておくと良い
* ```.gitignore```に書いておくとよいファイル名がまだわからない

## rustのドキュメント
https://doc.rust-jp.rs/book-ja/title-page.html
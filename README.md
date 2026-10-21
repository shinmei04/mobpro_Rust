# mobpro_Rust

このリポジトリはRust学習記録です

## 起動

Docker Desktopを起動し、プロジェクトのディレクトリで実行します。

```sh
cd mobpro_Rust
docker compose up -d --build
docker compose exec rust bash
```

## 実行

`src/main.rs` を編集し、コンテナ内のターミナルで実行します。

```sh
cargo run
```

## 終了

コンテナ内のシェルを終了してから、Mac側でコンテナを停止します。

```sh
exit
docker compose stop
```

コンテナ・データ・ビルドキャッシュは残ります。次回も同じ起動手順で再開できます。

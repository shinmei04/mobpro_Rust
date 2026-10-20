# mobpro_Rust

## 起動

Docker Desktopを起動し、プロジェクトのディレクトリで実行します。

```sh
cd mobpro_Rust
docker compose up -d --build
```

## 実行

`src/main.rs` を編集し、次のコマンドで実行します。

```sh
docker compose exec rust cargo run
```

## 終了

```sh
docker compose stop
```

コンテナ・データ・ビルドキャッシュは残ります。次回も同じ起動手順で再開できます。

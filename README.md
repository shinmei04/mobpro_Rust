# mobpro_Rust

RustをDocker内でビルド・実行する開発環境です。MacへのRust・Cargoのインストールは不要です。
Rust、rustfmt、Clippyは開発用イメージ内に置き、依存ライブラリとビルド結果はDocker管理のボリュームに保存します。
シェル設定やMac側のRust環境は変更しません。

## 実行する

Docker Desktopを起動し、クローンしたプロジェクトのディレクトリで実行します。

```sh
cd mobpro_Rust
docker compose up -d --build
docker compose exec rust cargo run
```

`Hello from Rust in Docker!` と表示されたら成功です。`src/main.rs` を編集し、同じ実行コマンドで動かせます。
初回ビルド時はRustの公式イメージと追加コンポーネントをダウンロードします。
コンテナはバックグラウンドで待機し、プログラムの終了後も残ります。
次回以降の起動は `docker compose up -d`、作業終了時の停止は `docker compose stop` です。

## よく使うコマンド

すべて、このプロジェクトのディレクトリで、コンテナを起動してから実行します。

```sh
# プログラムを実行
docker compose exec rust cargo run

# ビルドだけ実行
docker compose exec rust cargo build

# テスト
docker compose exec rust cargo test

# コードの整形
docker compose exec rust cargo fmt

# 静的チェック
docker compose exec rust cargo clippy -- -D warnings

# コンテナのシェルに入る（終了は exit）
docker compose exec rust bash
```

Mac側のターミナルで直接 `cargo` を実行する必要はありません。
生成される実行ファイルはLinux用です。実行もDocker内で行います。

## データの保存先

Mac側のプロジェクト全体を、コンテナ内の `/workspace` と共有しています。
コンテナ内でここに書き込むと、Mac側の同じファイルにも反映されます。

| 内容 | コンテナ内 | Mac側／Docker上の保存場所 | `docker compose stop` 後 |
| --- | --- | --- | --- |
| ソースコード・設定・Cargo.lock | `/workspace/` | クローン先の `mobpro_Rust/` | 残る |
| 保存したい入出力データ | `/workspace/data/` | クローン先の `mobpro_Rust/data/` | 残る |
| Cargoの依存ライブラリ・追加ツールなど | `/cargo/` | Dockerボリューム `mobpro-rust_cargo-cache` | 残る |
| ビルド結果・実行ファイル | `/target/` | Dockerボリューム `mobpro-rust_build-cache` | 残る |
| Rust本体・整形／静的チェックツール | `/usr/local/rustup/` など | 開発用イメージ `mobpro-rust-dev:local` とベースイメージ | 残る |
| それ以外のコンテナ内ファイル | `/tmp/`、`/root/` など | コンテナ内 | 残る。ただしコンテナ削除・作り直しで消える |

`docker compose up -d` は、設定やイメージに変更がなければ既存のコンテナを再利用します。
設定変更やイメージ更新によってコンテナが作り直されることがあるため、大切なデータは `data/` に保存してください。

例えばRustコードで `std::fs::write("data/result.txt", "結果")` を実行すると、
Mac側のプロジェクト内の `data/result.txt` に保存されます。
`data/` の内容はGit管理から除外しています（空フォルダ維持用の `.gitkeep` のみ管理）。
ソースコードと `Cargo.lock` はGit管理対象です。

Dockerのボリュームとイメージも、物理的にはMacのディスク容量を使います。
Docker DesktopのLinux VM用ディスク内で管理され、Macの `~/.cargo` やプロジェクト内の `target/` には保存しません。
ディスクの実際の保存先はDocker Desktopの **Settings → Resources → Advanced → Disk image location** で確認できます。
Macでの一般的な保存先は `$HOME/Library/Containers/com.docker.docker/Data/vms/0/data/Docker.raw` です。
このファイルはDockerが管理するため、直接編集・削除せず、以下のDockerコマンドで片付けてください。
ボリュームの確認は次のコマンドで行えます。表示される `Mountpoint` はDockerのLinux VM内のパスです。

```sh
docker volume inspect mobpro-rust_cargo-cache mobpro-rust_build-cache
docker system df
```

## 停止・再開

普段はコンテナを削除せず、停止して残しておきます。
シェルから `exit` しても待機用のコンテナは動き続けるので、作業を終えるときに停止します。

```sh
# 停止する（コンテナ・データ・キャッシュを保持）
docker compose stop

# 次回の起動（変更がなければ同じコンテナを再利用）
docker compose up -d

# 起動中のコンテナでプログラムを実行
docker compose exec rust cargo run

# コンテナの状態を確認（停止中も表示）
docker compose ps -a
```

## 不要になったときの削除

以下は環境を片付けたい場合だけ使います。

```sh
# このプロジェクトのコンテナ・ネットワークを削除（キャッシュは保持）
docker compose down

# このプロジェクトのコンテナ・ネットワーク・ボリュームを削除
docker compose down --volumes

# 開発用イメージも削除したい場合
docker image rm mobpro-rust-dev:local
```

`--volumes` は依存ライブラリや `cargo install` で追加したツール、ビルド結果を消すため、次回は取得・ビルドし直します。
これらのコマンドでMac側のソースコードや `data/` のファイルは削除されません。
ベースイメージやDockerのビルドキャッシュは別途Docker Desktop内に残ることがあります。
共有のイメージやキャッシュを一括削除するコマンドは、他のプロジェクトにも影響するためここには含めていません。

## Rustの更新

ベースには公式の `rust:1-bookworm` を使っています。`1` タグは更新されるため、再ビルド時期によってRustのバージョンが変わり得ます。
最新のベースイメージを取得して作り直す場合は次を実行します。

```sh
docker compose build --pull
docker compose up -d
docker compose exec rust rustc --version
```

再現性のためにバージョンを固定したい場合は、Dockerfileの `FROM` を具体的なRustバージョンのタグ、またはイメージのdigestに変更します。

## 公開リポジトリに含めるファイル

コード、Dockerの設定、`Cargo.lock`、このREADMEを管理します。
実行データ、ビルド結果、`.env`（`.env.example` を除く）、認証情報・秘密鍵、個人用エディタ設定は `.gitignore` で除外します。
`.env.example` を作る場合は、実際のパスワードやトークンを入れず、設定項目の例だけを記載してください。
秘密情報をソースコードやREADMEへ直接書いた場合は `.gitignore` では除外できないため、コミットするファイルの内容も確認してください。

## 参考

- [Docker: bind mounts](https://docs.docker.com/engine/storage/bind-mounts/)
- [Cargo Home（キャッシュの保存場所）](https://doc.rust-lang.org/cargo/guide/cargo-home.html)
- [Docker Desktopの設定](https://docs.docker.com/desktop/settings-and-maintenance/settings/)

# cologne_pg
Cologne phonetics integration for postgres via pgrx

## Try out
Just clone this repository and run `cargo pgrx run` if you have pgrx installed.
```
git clone https://github.com/DrSloth/cologne_pg
cd cologne_pg
cargo pgrx run
```

for installation of `pgrx` see https://github.com/pgcentralfoundation/pgrx

## Build and deploy
Use `cargo pgrx package` to build the postgres packages.
Either change the version in the `Cargo.toml`s `features.default` field or use `--no-default-features`
and set the version with `--features` you must have the appropriate postgres version installed.

For instance when building for postgres 16.
```
cargo pgrx package --no-default-features --features pg16
```

Then in `./target/release/cologne_pg-pg<pgversion>` you will find directories.
Copy the files in those directories to matching paths on the target device.

For example, again considering postgres 16:
```
cp target/release/cologne_pg-pg16/usr/lib/postgresql/16/lib/cologne_pg.so /usr/lib/postgresql/16/lib/
cp target/release/cologne_pg-pg16/usr/share/postgresql/16/extension/cologne_pg* /usr/share/postgresql/16/extension/
```

## Examples
```
cologne_pg=# CREATE EXTENSION cologne_pg;
CREATE EXTENSION
cologne_pg=# SELECT COLOGNE('MORELLO');
 cologne
---------
 675
(1 row)
```


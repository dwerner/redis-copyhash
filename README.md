# `redis-copyhash` 

Note that native modules are only supported in redis-4.0+ (And this module requires 4.0rc3+)

A redis module for quickly copying a hash from one key to another. While there are lua scripts around which do this, I thought it was low-hanging fruit for redis modules. Probably not the most production-ready, well-tested, or easy to read code, just a tool I slammed out in an hour or so.

This module was implemented using [redis-cell](https://github.com/brandur/redis-cell) and for now directly references [my fork](https://github.com/dwerner/redis-cell) until a crate is published.


Building:
```
cargo build --release

# install:
cp target/release/libredis_copyhash.so <redis-plugin-dir>
```

Configuration:

Add the following line to your redis.conf
```
loadmodule <redis-plugin-dir>/libredis_copyhash.so
```

Example usage:
```
127.0.0.1:6379> hgetall h1
1) "key"
2) "val"
3) "key2"
4) "val2"
5) "key3"
6) "val3"
7) "num"
8) "1"
127.0.0.1:6379> ch.copyhash
(error) Cell error: Wrong number of arguments to command.
Usage: ch.copyhash <src_key> <target_key>
127.0.0.1:6379> ch.copyhash h1 h2
"OK - 4 fields copied from h1 to h2"
127.0.0.1:6379> hgetall h2
1) "key"
2) "val"
3) "key2"
4) "val2"
5) "key3"
6) "val3"
7) "num"
8) "1"
```

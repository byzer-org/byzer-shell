# Byzer-shell

Byzer-shell is interactive command line tools for user who want to use Byzer-lang.

## Build

```
cargo build --release
```

## Usage

In Byzer-lang all-in-one release, you can run command like following 
in the root directory: 

```
./bin/byzer-shell --conf ./conf/byzer.conf
```

You can configure the byzer-shell in `./conf/byzer.conf`:

```
# Engine url 
# engine.url=http://remote

# Engine memory
engine.memory=6048m

# Byzer config
engine.streaming.spark.service=false

# Runtime config
engine.spark.shuffle.spill.batchSize=1000
```

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

## Example

Download the Byzer-lang all-in-one release:

1. [Mac](https://download.byzer.org/byzer/nightly-build/byzer-lang-darwin-amd64-3.0-latest.tar.gz)
2. [Linux](https://download.byzer.org/byzer/nightly-build/byzer-lang-linux-amd64-3.0-latest.tar.gz)
3. [Windows](https://download.byzer.org/byzer/nightly-build/byzer-lang-win-amd64-3.0-latest.tar.gz)

untar and then copy the `byzer-shell` to directory `bin`.

Now you can try some code like following:

```
(base) [w@me byzer-lang-darwin-amd64-3.0-2.2.2]$ ./bin/byzer-shell --conf conf/.mlsql.config
Conf file: "conf/.mlsql.config"
["-cp", "/Users/allwefantasy/Softwares/byzer-lang-darwin-amd64-3.0-2.2.2/main/*:/Users/allwefantasy/Softwares/byzer-lang-darwin-amd64-3.0-2.2.2/libs/*:/Users/allwefantasy/Softwares/byzer-lang-darwin-amd64-3.0-2.2.2/plugin/*:/Users/allwefantasy/Softwares/byzer-lang-darwin-amd64-3.0-2.2.2/spark/*", "streaming.core.StreamingApp", "-streaming.thrift", "false", "-streaming.mlsql.script.owner", "admin", "-streaming.driver.port", "9003", "-streaming.name", "Byzer-shell", "-streaming.master", "local[*]", "-streaming.plugin.clzznames", "tech.mlsql.plugins.ds.MLSQLExcelApp,tech.mlsql.plugins.shell.app.MLSQLShell,tech.mlsql.plugins.assert.app.MLSQLAssert", "-streaming.datalake.path", "./data", "-streaming.spark.service", "true", "-streaming.platform", "spark", "-streaming.job.cancel", "true", "-streaming.rest", "true"]
>
>
>
>
> load excel.`/Users/allwefantasy/projects/mlsql-example-project/example-data/excel/hello_world.xlsx`
> where header="true"
> as hello_world;
Executing....
┌───────┐
│ hello │
├───────┤
│ world │
└───────┘
>
> select count(*) from hello_world as output;
Executing....
┌──────────┐
│ count(1) │
├──────────┤
│ 1        │
└──────────┘
>
```

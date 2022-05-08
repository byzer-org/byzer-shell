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
(base) [w@me byzer-lang-darwin-amd64-3.0-2.2.2]$ ./bin/byzer-shell
Byzer-lang interpreter is starting...

 _                                                 _              _   _
| |__    _   _   ____   ___   _ __           ___  | |__     ___  | | | |
| '_ \  | | | | |_  /  / _ \ | '__|  _____  / __| | '_ \   / _ \ | | | |
| |_) | | |_| |  / /  |  __/ | |    |_____| \__ \ | | | | |  __/ | | | |
|_.__/   \__/ | /___|  \___| |_|            |___/ |_| |_|  \___| |_| |_|
         |___/


version: "2.2.2"
buildBy: "root"
date: "2022-04-01T06:18Z"
srcChecksum: "e4d2338da4bb8ed7b21c6902dcab119"
revision: "f580c01f18bf7b3903c9b2b0a05d579c908ae88e"
branch: "master"
url: "https://github.com/byzer-org/byzer-lang.git"
core: "3.1.1"

Type "CTRL-C" or "CTRL-D" to exit the program.

>>
load excel.`/Users/allwefantasy/projects/mlsql-example-project/example-data/excel/user-behavior.xlsx`
where header="true" as user_behavior;

select cast(datatime as date) as day,
       sum(case when behavior_type = 'pv' then 1 else 0 end) as pv,
       count(distinct user_id) as uv
from user_behavior
group by cast(datatime as date)
order by day
as day_pv_uv;



┌────────────┬──────┬─────┐
│ day        │ pv   │ uv  │
├────────────┼──────┼─────┤
│ 2017-11-25 │ 963  │ 720 │
├────────────┼──────┼─────┤
│ 2017-11-26 │ 946  │ 741 │
├────────────┼──────┼─────┤
│ 2017-11-27 │ 945  │ 723 │
├────────────┼──────┼─────┤
│ 2017-11-28 │ 941  │ 688 │
├────────────┼──────┼─────┤
│ 2017-11-29 │ 841  │ 682 │
├────────────┼──────┼─────┤
│ 2017-11-30 │ 906  │ 724 │
├────────────┼──────┼─────┤
│ 2017-12-01 │ 1023 │ 776 │
├────────────┼──────┼─────┤
│ 2017-12-02 │ 1182 │ 954 │
├────────────┼──────┼─────┤
│ 2017-12-03 │ 1196 │ 957 │
└────────────┴──────┴─────┘
```

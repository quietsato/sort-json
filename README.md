# sort-json

Sort json keys

```sh
echo '{
  "bbb": 2,
  "ccc": 3,
  "aaa": 1,
  "eee": 5,
  "ddd": 4
}' | sort-json
# => {"aaa":1,"bbb":2,"ccc":3,"ddd":4,"eee":5}
```

## Usage

```
sort-json 
Sort json keys

USAGE:
    sort-json [OPTIONS]

OPTIONS:
    -h, --help    Print help information
    -p            Output pretty json
```

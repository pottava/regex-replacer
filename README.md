
Supported tags and respective `Dockerfile` links:  
・latest ([Dockerfile](https://github.com/pottava/regex-replacer/blob/master/Dockerfile))

## Usage

`docker-compose run --rm replace`

with a docker-compose.yml.

```
version: "2.1"

services:
  replace:
    image: pottava/regex-replacer
    volumes:
      - .:/app
    environment:
      - DELIMITER=,
      - REPLACE_DELIMITER==
      - REPLACE=#\{FOO\}=foo,@BAR=bar
      - FILES=a.yaml,b.yaml
```

# confd-rs

`confd-rs` is a rust implementation of [kelsey hightower's confd](https://github.com/kelseyhightower/confd).
It is a hobby experiment and **should not be used in production**


## About confd

`confd` is a lightweight configuration management tool focused on:

* keeping local configuration files up-to-date using data stored in [etcd](https://github.com/coreos/etcd),
  [consul](http://consul.io), [dynamodb](http://aws.amazon.com/dynamodb/), [redis](http://redis.io),
  [vault](https://vaultproject.io), [zookeeper](https://zookeeper.apache.org), [aws ssm parameter store](https://aws.amazon.com/ec2/systems-manager/) or env vars and processing [template resources](docs/template-resources.md).
* reloading applications to pick up new config file changes

## High Level Architecture

* Input

** CLI
- get the basic cli data we need
  - scheme
  - url
  - username
  - password
- clap already handles the data format, and help message.
- based on the data, call the

** Toml or configuration file reader
- can read the toml file
- can enforce/validate expectations

* Integrations
- connection
- fetch keys
- watch for keys update
  * Redis
  * DynamoDB
  * Consul
  * etcd
* Env Writer
* Logging system


## TODO

- Define required fields, associated groups and required groups.

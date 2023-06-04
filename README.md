# confd-rs

`confd-rs` is a rust implementation of [kelsey hightower's confd](https://github.com/kelseyhightower/confd).
It is a hobby experiment and **should not be used in production**


## About confd

`confd` is a lightweight configuration management tool focused on:

* keeping local configuration files up-to-date using data stored in [etcd](https://github.com/coreos/etcd),
  [consul](http://consul.io), [dynamodb](http://aws.amazon.com/dynamodb/), [redis](http://redis.io),
  [vault](https://vaultproject.io), [zookeeper](https://zookeeper.apache.org), [aws ssm parameter store](https://aws.amazon.com/ec2/systems-manager/) or env vars and processing [template resources](docs/template-resources.md).
* reloading applications to pick up new config file changes

## High Level Architecture - MVP

* Input

** CLI
- get the basic cli data we need
  - scheme
  - url
  - username
  - password
- clap already handles the data format, and help message.
- based on the data, call the

* Integrations - etcd, vault, redis
- connection
- fetch keys

* Main engine - a PubSub system
- every integrator fetches data, and pushes it to the pubsub system or alternatively, the integrators commit their connection to the pub-system for long-polling.


- Env writer subscribes to the pubsub system, and gets the data it needs

* Env Writer
* Logging system

* Watch
- watch for keys update

** Toml or configuration file reader
- can read the toml file
- can enforce/validate expectations


## TODO

- Define required fields, associated groups and required groups.

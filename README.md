# Mesos Statistics Publisher

This is a simple utility program to publish Apache Mesos task statistics to a Kafka topic in [Influx line protocol](https://docs.influxdata.com/influxdb/v0.9/write_protocols/line/).

It uses [miniMesos](https://minimesos.org/) for testing.

## Usage

Spin up the Mesos cluster:

```
minimesos up
```

## Deploying Marathon Apps

```
container-transform -i compose -o marathon docker-compose.yml > marathon.json
```

## Links

* [marathon-deployer](https://github.com/avast/marathon-deployer)
* [container-transform](https://github.com/micahhausler/container-transform)
* [DanielKeep/modules.md](https://gist.github.com/DanielKeep/470f4e114d28cd0c8d43)

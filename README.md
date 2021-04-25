# RedisLess
RedisLess is an embedded and scalable in-memory Redis-like library that act as a Redis Server and is compatible with the Redis API.

> RedisLess is the concatenation of Redis + Serverless.

## Use cases
Developers often use Redis to share states between our apps. The problem is that we need to set up a Redis server and manage it. The idea of RedisLess is to bring you the ease of use of Redis without the inconvenience (managing a server).

What RedisLess is:

* Built with performance in mind.
* Sharing data and states between your apps.
* Compatible with the Redis API (RESP).

What RedisLess is not:

* Production ready (we are working on)
* 1:1 Redis Server implementation

# Usage

## Library
## Python

### Install
```bash
pip install redisless # RedisLess library with Python binding
pip install redis # redis client
```

### Usage
```python
from redisless import RedisLess
import redis

redisless = RedisLess()

# start RedisLess embedded local server
redisless.start_server()

# Connect to RedisLess on localhost:16379
redis = redis.Redis(host='localhost', port=16379, db=0)

redis.set('foo', 'bar')
redis.get('foo') # return bar 

# stop RedisLess embedded local server
redisless.stop_server()
```

# Features
- [ ] Compatible with the Redis API ([see details](REDIS_FEATURES.md))
- [ ] Redis Server API compatible
- [ ] Atomic operations
- [ ] Multi-threaded
- [ ] Memory off-load and disk persistence
- [ ] Cluster mode with replication

# Supported clients
- [ ] Python
- [ ] Golang
- [ ] Java

# Performance
TODO

# Contribution welcome!
TODO

# References

- Redis Internals: [Free e-book](https://redislabs.com/ebook)
- [Redis Protocol Specification](https://redis.io/topics/protocol) (RESP)
- [Sonic](https://github.com/valeriansaliou/sonic): fast and lightweight Elasticsearch alternative
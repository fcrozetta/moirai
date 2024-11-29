# moirai-engine

[![PyPI - Version](https://img.shields.io/pypi/v/moirai-engine.svg)](https://pypi.org/project/moirai-engine)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/moirai-engine.svg)](https://pypi.org/project/moirai-engine)

-----

## Table of Contents
<!-- TOC -->

- [moirai-engine](#moirai-engine)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Usage](#usage)
    - [Basic Example](#basic-example)
    - [Advanced Example](#advanced-example)
  - [About](#about)
  - [License](#license)

<!-- /TOC -->


> **NOTE:** This doc is still in progress. for now it was generated by AI (sorry).

## Installation

```console
pip install moirai-engine
```

## Usage

### Basic Example

Here is a basic example of how to use the `moirai-engine` library to create and run a simple job:

```python
import time
from moirai_engine.core.engine import Engine
from moirai_engine.utils.samples import hello_world

# Create and start the engine
engine = Engine()
engine.start()

# Create a job using the hello_world sample
job = hello_world()

# Add the job to the engine
engine.add_job(job)

# Let the engine run for a while
time.sleep(5)

# Stop the engine
engine.stop()
```

### Advanced Example

Here is an advanced example that includes real-time notifications and job cancellation:

```python
import time
from moirai_engine.core.engine import Engine
from moirai_engine.utils.samples import hello_world, slow_hello_world

def notification_listener(notification):
    print(f"Received notification: {notification}")

# Create and start the engine with notification listener
engine = Engine(max_workers=4)
engine.add_notification_listener(notification_listener)
engine.start()

# Create jobs
job = slow_hello_world()
job2 = hello_world()

# Add jobs to the engine
engine.add_job(job)
engine.add_job(job2)

# Start notification listeners for the jobs
engine.start_notification_listener(job.id)
engine.start_notification_listener(job2.id)

# Let the engine run for a while
time.sleep(2)


# Let the engine run for a while
time.sleep(2)

# Stop the engine
engine.stop()
```

## About

`moirai-engine` is the engine that powers [Ananke](https://github.com/fcrozetta/ananke), but it was planned to be used as a processing engine. This project is still in progress, and features like task registry and autodiscover are still in development.

## License

MIT License

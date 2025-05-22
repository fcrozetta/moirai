# moirai

> **WIP**: This is still a work in progress.

## TODO:
- plugin.json example

Moirai is a workflow engine to be used in python application (ananke). 

There are two main principles that I looked for in other engines, but couldn't find:

1. Community driven plugins
   1. point to a git repo and branch/tag
2. Connect specific attributes (inputs and outputs) instead of passing generic json variables

As a side note: I really want any dev with any language to be able to build plugins. Initially as a virtual environment, but later i plan to use docker as environment.


## Moirai Architecture Overview

A high-level design and separation of concerns for the **Moirai** workflow engine. This document focuses on internal components, their responsibilities, and how they interact.

---

### 1. Configuration Loader

* **Reads** `moirai.config.toml` on startup
* **Parses** engine settings and plugin declarations
* **Provides** a config object to all other components
* **Optional**: watches for changes and triggers reloads

---

### 2. Plugin Manager

* **Responsibilities**:

  * Discover local and Git-based plugins as declared in config
  * Validate each `plugin.json` manifest
  * Load and parse `types/*.json` and `nodes/*.json` into memory
  * Maintain a registry: `HashMap<plugin_fqdn, Plugin>`
  * Handle install/remove operations (update config, clone/delete folders)

---

### 3. Workflow Validator

* **Responsibilities**:

  * Validate workflow definitions (if used)
  * Ensure all referenced `node_fqdn` and `type_fqdn` exist in loaded plugins
  * Check node I/O schemas: required inputs, type compatibility
  * Detect graph errors: missing nodes, broken edges, cycles in non-flow nodes
  * Provide clear error messages for misconfigurations

---

### 4. Engine Core

* **Responsibilities**:

  * Provide `start()` and `stop()` methods for the runtime
  * Accept `run_workflow(workflow_definition_or_id)` calls via library or CLI
  * Schedule node execution respecting dependency order and parallelism limits
  * Enforce timeouts and failure policies
  * Coordinate between Plugin Manager, Node Executor, and Logging

---

### 5. Node Executor

* **Responsibilities**:

  * Given a `NodeSpec` and input values, launch the node process:

    * Use the plugin’s declared `execution` settings (language, method, entrypoint)
  * Stream stdout as NDJSON events (`log`, `progress`, `output`, `error`)
  * Parse and dispatch events immediately
  * Handle subprocess lifecycle: kill on timeout, propagate failures
  * Return an in-memory `NodeResult` object to the Engine Core

---

### 6. Logging & Transient Storage

* **Responsibilities**:

  * Receive streaming events from Node Executor
  * Stream events to console/UI in real time
  * Optionally dump raw NDJSON to `runs/<run_id>/` for debugging
  * Do *not* persist long-term (Ananke handles durable storage)

---

### 7. Separation of Concerns

| Module               | Primary Role                                           |
| -------------------- | ------------------------------------------------------ |
| Configuration Loader | Load and watch config                                  |
| Plugin Manager       | Discover, validate, and register plugins               |
| Workflow Validator   | Static validation of workflows                         |
| Engine Core          | Orchestrate node execution, scheduling, error handling |
| Node Executor        | Launch and monitor node processes                      |
| Logging              | Real-time streaming and transient storage of events    |

---

### 8. Next Design Topics

* **Scheduler Integration**: where cron/webhooks live (Ananke)
* **Artifact Management**: per-run directories, cleanup policy
* **Extensible Execution Backends**: MQ or HTTP for advanced nodes
* **Testing & Development Tools**: `moirai-dev`, schema linting, mock runner

---



## Concepts

- Moirai focus is the execution engine, and therefore won't focus in schedulers, webhooks, or CLI triggers
- Files are being used for configurations and plugins, but may be changed later

## Plugin system

### PLugin structure

``` text
myplugin/
│
├── plugin.json               # Manifest file (always at the root)
│
├── types/                    # All data type definitions
│   ├── string.json
│   ├── number.json
│   └── json.json
│
├── nodes/                    # Node definitions
│   ├── log.json
│   ├── start.json
│   └── end.json
│
├── lib/                      # Optional: shared libraries/helpers for plugin nodes
│   └── utils.py
│
├── src/                      # Actual node implementations (Python, Rust, etc.)
│   ├── log.py
│   ├── start.py
│   └── end.py
│
├── venv/                     # Isolated virtual environment (generated locally)
│   └── ...
│
└── README.md                 # Plugin documentation
```
### Plugin json

plugin.json
```json
{
  "plugin_fqdn": "mysys",
  "version": "1.0.0",
  "metadata": {
    "author": "Alice",
    "created": "2025-05-22",
    "tags": ["example", "demo"]
  },
  "runtime": {
    "language":"python",
    "version":"3.12",
    "manager":"uv"
  },
  "types": [
    "types/string.json",
    "types/number.json"
  ],
  "nodes": [
    "nodes/log.json",
    "nodes/uppercase.json"
  ]
}

``` 

### Data type definitions

#### number type

```json
{
  "type_fqdn": "sys:number",
  "version": "1.0.0",

  "metadata": {
    "display_name": "Number",
    "description": "A floating-point number",
    "author": "F.H. Crozetta",
    "created": "2025-05-07T00:00:00Z"
  },

  "properties": {
    "kind": "primitive",
    "format": "application/x-float",
    "constraints": {
      "min_value": -1.0e12,
      "max_value": 1.0e12
    }
  }
}

```

#### json type

```json
{
  "type_fqdn": "sys:json",
  "version": "1.0.0",

  "metadata": {
    "display_name": "JSON Object",
    "description": "A generic JSON structure of arbitrary complexity",
    "author": "F.H. Crozetta",
    "created": "2025-05-07T00:00:00Z"
  },

  "properties": {
    "kind": "primitive",
    "format": "application/json",
    "constraints": {}
  }
}
```

#### String type

```json
{
  "type_fqdn": "sys:string",
  "version": "1.0.0",

  "metadata": {
    "display_name": "String",
    "description": "A UTF-8 encoded text string",
    "author": "F.H. Crozetta",
    "created": "2025-05-07T00:00:00Z"
  },

  "properties": {
    "kind": "primitive",
    "format": "text/plain",
    "constraints": {}
  }
}
```

### Action Node definitions

#### Log definition

```json
{
  "type": "action",
  "node_fqdn": "sys:log",
  "plugin_version": "1.0.0",
  "node_version": "1.0.0",

  "inputs": [
    {
      "name": "message",
      "type": "sys:string",
      "multiple": false,
      "required": true
    }
  ],
  "outputs": [],

  "metadata": {
    "display_name": "Log Message",
    "description": "Logs the given string message to standard output or a logging system.",
    "author": "F.H. Crozetta",
    "created": "2025-05-07T00:00:00Z",
    "tags": ["logging", "debug"],
    "icon": "log_icon"
  }
}
```

### Flow Node Definitions

#### Start Node

```json
{
  "type": "flow",
  "node_fqdn": "sys:start",
  "plugin_version": "1.0.0",
  "node_version": "1.0.0",

  "inputs": [],
  "outputs": [],

  "metadata": {
    "display_name": "Start",
    "description": "Entry point for workflow execution. Always the first node.",
    "author": "F.H. Crozetta",
    "created": "2025-05-07T00:00:00Z",
    "tags": ["flow", "entry"],
    "icon": "start_icon"
  }
}
```

#### End Node

```json
{
  "type": "flow",
  "node_fqdn": "sys:end",
  "plugin_version": "1.0.0",
  "node_version": "1.0.0",

  "inputs": [
    {
      "name": "return_code",
      "type": "sys:number",
      "multiple": false,
      "required": false
    },
    {
      "name": "message",
      "type": "sys:string",
      "multiple": false,
      "required": false
    }
  ],
  "outputs": [],

  "metadata": {
    "display_name": "End",
    "description": "Terminates workflow execution. Can receive a return code and message for summary output or external signaling.",
    "author": "F.H. Crozetta",
    "created": "2025-05-07T00:00:00Z",
    "tags": ["flow", "exit"],
    "icon": "end_icon"
  }
}
```

## Workflow design

```json
{
  "version": "1.0",
  "name": "example_workflow",

  "metadata": {
    "description": "Logs a static message and ends the workflow cleanly.",
    "author": "F.H. Crozetta",
    "created": "2025-05-07T00:00:00Z",
    "tags": ["demo", "logging", "example"]
  },

  "nodes": [
    {
      "id": "start1",
      "node_fqdn": "sys:start"
    },
    {
      "id": "msg1",
      "node_fqdn": "sys:string",
      "value": "Hello from Moirai!"
    },
    {
      "id": "log1",
      "node_fqdn": "sys:log"
    },
    {
      "id": "end1",
      "node_fqdn": "sys:end"
    }
  ],

  "edges": [
    {
      "from_node": "start1",
      "from_output": "on_success",
      "to_node": "log1",
      "to_input": "__flow__"
    },
    {
      "from_node": "msg1",
      "from_output": "value",
      "to_node": "log1",
      "to_input": "message"
    },
    {
      "from_node": "log1",
      "from_output": "on_success",
      "to_node": "end1",
      "to_input": "__flow__"
    }
  ]
}
```

## Engine Plugins file

### moirai.config.toml
```toml 
# Moirai Engine Configuration
version = "1.0.0"           # Config file format version

[engine]
engine_version = "0.1.0"    # Required engine version to run this setup
# Optional: Path to a workflow file (for CLI/testing/dev tooling)
workflow = "workflow.json"

# Optional engine-level execution parameters
concurrency = 4
timeout_seconds = 60

[plugins]

# Core system plugin
[plugins.sys]
path = "./plugins/sys"

# Git-based plugin
[plugins.image_tools]
git = "https://github.com/fcrozetta/moirai-plugin-image-tools"
rev = "main"

# Another local plugin
[plugins.azure]
path = "./plugins/azure"
```


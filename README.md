# moirai

> WIP

Moirai is a workflow engine to be used in python application (ananke). 

There are two main principles that I looked for in other engines, but couldn't find:

1. Community driven plugins
   1. point to a git repo and branch/tag
2. Connect specific attributes (inputs and outputs) instead of passing generic json variables

As a side note: I really want any dev with any language to be able to build plugins. Initially as a virtual environment, but later i plan to use docker as environment.

## Plugin system

### PLugin structure
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


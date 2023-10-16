# Service Orchestrator

Service orchestrator without the boilerplate.

## Workflow Architecture

Each endpoint of the orchestrator is defined with a workflow. A workflow has
discrete configuratable steps with narrowly-defined responsibility.

```mermaid
flowchart LR
    transformResponse["transform"]
    mapResponse["map"]

    request --> transform
    map --> service
    service --> transformResponse
    mapResponse --> response
    
    subgraph Workflow
        direction LR
        transform --> map
        transformResponse --> mapResponse
    end
```

The values from a service response can be used as part of the request to another
service.

```mermaid
flowchart LR
    request --> transform
    service --> transform
    map --> response

    subgraph Workflow
        direction LR
        transform --> map
    end
```

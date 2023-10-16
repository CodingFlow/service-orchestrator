# Service Orchestrator

Service orchestrator without the boilerplate.

## Workflow Architecture

Each endpoint of the orchestrator is defined with a workflow. A workflow has
discrete configuratable steps with narrowly-defined responsibility.

```mermaid
flowchart LR
    transformRequest["transform"]
    mapRequest["map"]
    transformResponse["transform"]
    mapResponse["map"]

    request --> transformRequest
    mapRequest --> service
    service --> transformResponse
    mapResponse --> response
    
    subgraph Workflow
        direction LR
        transformRequest --> mapRequest
        transformResponse --> mapResponse
    end
```

The values from a service response can be used as part of the request to another
service.

```mermaid
flowchart LR
    mapRequest["map"]
    serviceB["service"]

    request --> transform
    serviceB --> transform
    mapRequest --> service

    subgraph Workflow
        direction LR
        transform --> mapRequest
    end
```

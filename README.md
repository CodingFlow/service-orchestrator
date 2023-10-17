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

## Overall Architecture

1. Provide Open API specifications for orchestrator endpoints and dependent
   services
2. Create workflow specifications using domain-specific language

Given a Workflow Request/Response:

```yaml
Request
    input 1
    input 2
Response
    output 1
    output 2
    output 3
```

And a service called `Service A` with Request/Response:

```yaml
Request
    input A
Response
    output A
    output B
```

And a service called `Service B` with Request/Response:

```yaml
Request
    input A
    input B
Response
    output A
```

Potential Example of Workflow Specification:

```yaml
Workflow A:
    transform:
        inputSum: add(input 1, input 2)
    Service A:
        request:
            map:
                input A: inputSum
    Service B:
        request:
            map:
                input A: "Service A: input B"
                input B: input 2
    response:
        output1: "Service B: output A"
        output2: "Service A: output A"
        output3: "Service A: output B"
```

The sequence diagram of Workflow A:

```mermaid
sequenceDiagram
    API Consumer ->> Workflow: input 1, input 2

    Workflow ->> Service A: add(input 1, input 2)
    Service A -->> Workflow: output A, output B

    Workflow ->> Service B: Service A: input B, input 2
    Service B -->> Workflow: output A

    Workflow -->> API Consumer: Service B: output A, Service A: output A, Service A: output B
```

The sequence diagram with example concrete values:

```mermaid
sequenceDiagram
    API Consumer ->> Workflow: 1, 2

    Workflow ->> Service A: add(1, 2) = 3
    Service A -->> Workflow: 10, 11

    Workflow ->> Service B: 11, 2
    Service B -->> Workflow: 20

    Workflow -->> API Consumer: 20, 10, 11
```

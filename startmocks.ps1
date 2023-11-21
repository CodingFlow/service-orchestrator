start powershell {
    mockoon-cli start --data .\src\service_specs\dog_service.yaml --port 3000
}

start powershell {
    mockoon-cli start --data .\src\service_specs\meat_service.yaml --port 3001
}

start powershell {
    mockoon-cli start --data .\src\service_specs\cow_service.yaml --port 3003
}

start powershell {
    mockoon-cli start --data .\src\service_specs\oven_service.yaml --port 3002
}
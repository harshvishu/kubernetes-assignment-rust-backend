# Kubernetes Assignment

This project demonstrates the basic concepts of kubernetes.

## Prerequisites

Before getting started, ensure that you have the following prerequisites installed on your system:

- Docker
- Kubernetes (kubectl)
- Minikube (optional if using Minikube)

## Installation

1. Clone the repository:

```shell
    git clone https://github.com/harshvishu/kubernetes-assignment-rust-backend
```

2. Change to the project directory:

```shell
    cd kubernetes-assignment-rust-backend-main
```

3. Deploy the required resources:
   

```shell
    # Apply secrets and config maps
   kubectl apply -f deployments/db-secrets.yaml
   kubectl apply -f deployments/db-config.yaml
```
```shell
   # Apply postgres database service and deployment
   kubectl apply -f deployments/postgres-service.yaml
   kubectl apply -f deployments/postgres-persistence-volume-deployment.yaml
```

```shell
   # Run databse migration and initial seed data if-required
   kubectl apply -f deployments/db-migration-up-job.yaml
   kubectl apply -f deployments/db-seed-job.yaml
```


```shell
   # Start REST Api and Front-End App
   kubectl apply -f deployments/todo-config.yaml
   kubectl apply -f deployments/todo-api-deployment.yaml
   kubectl apply -f deployments/todo-api-service.yaml
   kubectl apply -f deployments/todo-web-deployment.yaml
   kubectl apply -f deployments/todo-web-loadbalancer-service.yaml
```

4. Verify the deployment:

```shell
   kubectl get pods
   kubectl get services
   kubectl get pv
   kubectl get pvc
```

## Usage
To access the application, follow these steps:

1. Get the external IP for the load balancer service (minikube):

```shell
   minikube service todo-web --url
```

2. Using Kubernetes on docker hub, simply go to:

```shell
   http://localhost:80
```

## Cleanup
To clean up the resources created by the application, run the following command:

```shell
   kubectl delete all --all
```

## Repositories used 

- [harshvishu/todo-web-image](https://hub.docker.com/repository/docker/harshvishu/todo-web-image/)
- [harshvishu/todo-api-image](https://hub.docker.com/repository/docker/harshvishu/todo-api-image/)
- [harshvishu/todo-migration-image](https://hub.docker.com/repository/docker/harshvishu/todo-migration-image/)
- [postgres](https://hub.docker.com/_/postgres)

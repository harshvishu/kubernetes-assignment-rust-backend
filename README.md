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
    git clone [repository-url]
```

2. Change to the project directory:

```shell
    cd [project-directory]
```

3. Deploy the required resources:
   
```shell
    kubectl apply -f deployments/db-secrets.yaml
    kubectl apply -f deployments/db-config.yaml
    kubectl apply -f deployments/postgres-persistence-volume-deployment.yaml
    kubectl apply -f deployments/postgres-service.yaml
    kubectl apply -f deployments/db-migration-up-job.yaml
    kubectl apply -f deployments/db-seed-job.yaml
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

## Cleanup
To clean up the resources created by the application, run the following command:

```shell
   kubectl delete -f deployments/
```

## Repositories used 

- harshvishu/todo-web-image
- harshvishu/todo-api-image
- harshvishu/todo-migration-image
- postgres
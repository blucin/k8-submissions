# Namespaces

This folder contains Kubernetes namespace manifests for organizing exercises and projects.

## Exercises Namespace

The `exercises.yaml` file defines the `exercises` namespace used by the following applications:
- Log Output
- Ping Pong

To create the namespace:

```bash
kubectl apply -f exercises.yaml
```

## Project Namespace

The `project.yaml` file defines the `project` namespace used by the following application:
- The Project (todo-app with todo-backend)

To create the namespace:

```bash
kubectl apply -f project.yaml
```

## Creating All Namespaces

To create both namespaces at once:

```bash
kubectl apply -f .
```

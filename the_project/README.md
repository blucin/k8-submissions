# The Project

This is a project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

## Getting Started

1. Make sure your local k3d cluster is running with required configuration
```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
```

2. Build the Docker image and import it to your local k3d cluster

```bash
docker build -t the-project .
k3d image import the-project:latest
```

3. Run the deployment using kubectl

```bash
kubectl apply -f manifests/deployment.yaml
kubectl apply -f manifests/service.yaml
kubectl apply -f manifests/ingress.yaml

# Test the page
http://localhost:8081
```
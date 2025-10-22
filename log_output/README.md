# Log Output

This is a sample application project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

## Getting Started

1. Make sure your local k3d cluster is running
```bash
k3d cluster create -a 2
```

2. Build the Docker image and import it to your local k3d cluster

```bash
docker build -t log-output .
k3d image import log-output:latest
```

3. Run the deployment using kubectl

```bash
kubectl apply -f manifests/deployment.yaml
kubectl apply -f manifests/service.yaml
kubectl apply -f manifests/ingress.yaml
```

4. Access the application from your browser at `http://localhost:8081`

Expected output
```bash
2025-10-22T15:25:41Z: a4b6ff12-e2dc-431e-a48e-077bb18f5044
```

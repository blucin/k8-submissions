# Log Output

This is a sample application project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

## Getting Started

1. Make sure your local k3d cluster is running
```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
```

2. Build the Docker images and import it to your local k3d cluster

```bash
docker build -f .\Dockerfile -t log-output:latest .
k3d image import log-output:latest
```

3. Run the deployment using kubectl

```bash
kubectl apply -f manifests/deployment.yaml
kubectl apply -f manifests/service.yaml
```

> Excerise did not recommend any method to expose the service to host machine, so I used port forwarding for simplicity.

4. Port forward the service to localhost

```bash
kubectl port-forward svc/log-output-service 5678:5678
```

5. Access the application from your browser at `http://localhost:5678/`

Expected output
```bash
2025-11-29T13:52:53Z: a08dbf9f-c459-407e-9fcc-610999c64d73
Pings / Pongs: 2
```

> [!NOTE]
> You might want to port forward the ping pong service as well and access it for incrementing the counts.
> Read here: ![Ping Pong README](https://github.com/blucin/k8-submissions/tree/2.1/ping_pong/README.md) 

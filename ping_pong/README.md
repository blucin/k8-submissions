# Ping Pong

This is a sample application project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

## Getting Started

1. Make sure your local k3d cluster is running
```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
```

2. Build the Docker image and import it to your local k3d cluster

```bash
docker build -t ping-pong .
k3d image import ping-pong:latest
```

3. Run the deployment using kubectl

```bash
kubectl apply -f manifests/deployment.yaml
kubectl apply -f manifests/service.yaml
```

> Excerise did not recommend any method to expose the service to host machine, so I used port forwarding for simplicity.

4. Port forward the service to localhost

```bash
kubectl port-forward svc/ping-pong-service 3000:3000
```

5. Access the application from your browser at `http://localhost:3000/pingpong`

Expected output
```
Error writing log file
```

It increments the in-memory counter behind the scenes each time you refresh the page. The error is expected as the excerise did not insist on cleanin up the old logic so I did not bother fixing it.



# Ping Pong

This is a sample application project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

## Getting Started

1. Make sure your local k3d cluster is running with the local registry enabled
```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2 --registry-create k3d-local-registry
```

2. Build the Docker image and push it to the local registry

```bash
docker build -t localhost:5000/ping-pong:latest .
docker push localhost:5000/ping-pong:latest
```

3. Make sure the exercises namespace exists

```bash
kubectl apply -f ../namespaces/exercises.yaml
```

4. Run the deployment using kubectl

```bash
kubectl apply -f manifests/deployment.yaml
kubectl apply -f manifests/service.yaml
```

> Exercise did not recommend any method to expose the service to host machine, so I used port forwarding for simplicity.

5. Port forward the service to localhost

```bash
kubectl port-forward svc/ping-pong-service 3000:3000 -n exercises
```

6. Access the application from your browser at `http://localhost:3000/pingpong`

Expected output
```
Error writing log file
```

It increments the in-memory counter behind the scenes each time you refresh the page. The error is expected as the exercise did not insist on cleaning up the old logic so I did not bother fixing it.



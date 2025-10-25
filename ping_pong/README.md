# Ping Pong

This is a sample application project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

This application uses a shared ingress resource from log_output:

- [Shared Ingress File](https://github.com/blucin/k8-submissions/tree/1.9/log_output/manifests/ingress.yaml)

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
kubectl apply -f manifests/
kubectl apply -f ../log_output/manifests/
```

4. Access the application from your browser at
- `http://localhost:8081/` -> `2025-10-25T08:16:42Z: cd099996-3106-41b6-9d23-304788951af3` (log_output)
- `http://localhost:8081/pingpong` -> `ping 0` (ping_pong)


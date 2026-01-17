# Log Output

This is a sample application project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

## Getting Started

1. Make a k3d local registry for image storage
```bash
k3d registry create local-registry --port 5000
```

2. Make sure your local k3d cluster is running and using the local registry
```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2 --registry-use k3d-local-registry:5000
```

3. Build the Docker image and push it to the local registry

```bash
docker build -f Dockerfile -t log-output:latest .
docker tag log-output:latest localhost:5000/log-output:latest
docker push localhost:5000/log-output:latest
```

> [!NOTE]
> Note that the image is tagged as `localhost:5000` when pushing from the host machine, but the Kubernetes manifests reference it as `k3d-local-registry:5000` since that's the internal cluster DNS name.

4. Create the exercises namespace

```bash
kubectl apply -f ../namespaces/exercises.yaml
kubens exercises
```

5. Run the deployment using kubectl

```bash
kubectl apply -f manifests/configmap.yaml
kubectl apply -f manifests/deployment.yaml
kubectl apply -f manifests/service.yaml
```

> Exercise did not recommend any method to expose the service to host machine, so I used NodePort with port forwarding for simplicity.

6. Port forward the service to localhost

```bash
kubectl port-forward svc/log-output-service 5678:5678 -n exercises
```

7. Access the application from your browser at `http://localhost:5678/`

Expected output
```
file content: it is what it is, a text from the file
env variable: MESSAGE=hello world
2026-01-17T12:21:11Z: 631ce58f-dce5-4018-9fcf-98283c58c592
Pings / Pongs: 2
```

> [!NOTE]
> You might want to port forward the ping pong service as well and access it for incrementing the counts.
> Read here: ![Ping Pong README](https://github.com/blucin/k8-submissions/tree/2.3/ping_pong/README.md) 

# Persisting data

This is a sample application project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

## Getting Started

1. Make sure your local k3d cluster is running
```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
```

2. Make sure the persistent volume mount location exists
```bash
docker exec k3d-k3s-default-agent-0 mkdir -p /tmp/kube
```

3. Create the required resources using kubectl

```bash
# Volumes
kubectl apply -f persistentvolume.yaml
kubectl apply -f persistentvolumeclaim.yaml

# Both the services: ping-pong and log-output
kubectl apply -f ../log_output/manifests/
kubectl apply -f ../ping_pong/manifests/
```

4. Make some requests on `localhost:8081/pingpong` so it writes to the log file
```bash
Ping / Pongs: 3
```

5. Expected output from `localhost:8081/`
```bash
2025-10-25T12:08:22Z: 70efa713-dbbe-475d-9a05-292921115d26
2025-10-25T12:08:27Z: 429817bc-cb5d-42e3-9ef2-c8d21fa3edf3
2025-10-25T12:08:32Z: c57b6de1-1e2e-4834-ba06-180943162341
2025-10-25T12:08:37Z: 32a36d08-eac5-4655-ad42-114c1f6cca7c
2025-10-25T12:08:42Z: 7e589ce4-e91d-4f1b-b5f1-15b5aa7f4660
Ping / Pongs: 0
2025-10-25T12:08:47Z: dc6bb48f-7664-421a-805a-e68cb61f1aa9
Ping / Pongs: 1
Ping / Pongs: 2
Ping / Pongs: 3
2025-10-25T12:08:52Z: 33172408-6a5b-45e8-970f-3cd23a50ea99
```

This demonstrates the shared state between the log-output and ping-pong service using local persistent volume.
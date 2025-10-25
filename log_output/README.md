# Log Output

This is a sample application project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

## Getting Started

1. Make sure your local k3d cluster is running
```bash
k3d cluster create --port 8082:30080@agent:0 -p 8081:80@loadbalancer --agents 2
```

2. Build the Docker images and import it to your local k3d cluster

```bash
docker build -f .\Dockerfile.reader -t log-output:reader .
k3d image import log-output:reader

docker build -f .\Dockerfile.writer -t log-output:writer .
k3d image import log-output:writer
```

3. Run the deployment using kubectl

```bash
kubectl apply -f manifests/
```

4. Access the application from your browser at `http://localhost:8081`

Expected output
```bash
2025-10-25T11:16:13Z: dcdb416f-f1f8-4bbf-9553-0b69eefbd75c
2025-10-25T11:16:18Z: a313308f-18bc-4bb5-bf9e-e9c0155c0087
2025-10-25T11:16:23Z: 4708c829-d7e1-46ba-a883-b1a57715eeb1
2025-10-25T11:16:28Z: 1082337c-39d0-440d-8934-1f60fa62510b
2025-10-25T11:16:33Z: 8bb87bae-5268-4fce-aaa8-2add75662eb6
2025-10-25T11:16:38Z: 5c01c1c4-1191-40a4-a927-38fca8ea986a
...
```

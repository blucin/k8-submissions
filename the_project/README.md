# The Project

This is a project to learn kubernetes from the MOOC.fi course "DevOps with Kubernetes".

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
kubectl apply -f ../volumes/

# The Project
docker build -t the-project .
k3d image import the-project:latest
kubectl apply -f ../the_project/manifests/
```

4. Visit `localhost:8081/`

![The Project](../assets/the_project_screenshot.png)

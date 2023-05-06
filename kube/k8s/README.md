# Running Benchmarks

1. Initialize the data directory by running the following:

  ```bash
  ./benchmarks/tpch-gen.sh
  ```

2. Mount your local data directory into your running minikube cluster:

  ```
  minikube mount /Users/bog/Documents/promise/arrow-ballista/benchmarks/data:/benchmarks/data
  ```

3. Run skaffold dev in root

  ```
  skaffold dev
  ```

4. Identify the Benchmarks pod by running `kubectl get pods`

5. SSH into the pod by running:

  ```
  kubectl exec -it pod/{pod-identifier} -- /bin/bash
  ```

5. Run the test script: `./run.sh`

## Basic webserver to handle the Meilisearch webhook

Meilisearch introduced a webhook to be notified every time a task is processed.
See the whole documentation here: https://www.meilisearch.com/docs/learn/async/task_webhook

This repository shows how to receive tasks with the bare minimum requirements.

### Usage

Run this code first:
```bash
cargo run
```

Then run Meilisearch on the same machine with the following arguments:
```bash
./meilisearch --task-webhook-url http://localhost:9000/
```

Finally, process a task, and you'll see the payload being deserialized.

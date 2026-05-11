# Testing — `feat/get-medication-plans`

Instructions for an AI agent (or human) to verify the new
`GET /api/medication/plans` endpoint end-to-end. Self-contained:
brings up a throwaway MongoDB in Docker, runs the backend against
it, seeds known data, and asserts each test case.

## Prerequisites

- Docker daemon reachable (no compose needed; this uses `docker run`).
- Rust toolchain (`cargo`) able to build the backend. On a host
  without `pkg-config` but with OpenSSL headers/libs present, set:
  ```
  export OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu
  export OPENSSL_INCLUDE_DIR=/usr/include
  export OPENSSL_NO_VENDOR=1
  ```
- `curl` and `jq` for invoking and parsing responses.

> The native-binary path below is the primary, fast feedback loop.
> A separate end-to-end smoke test against `docker-compose` is in
> section 6.

## 1. Start MongoDB

```bash
docker run -d --rm --name medman-test-mongo \
  -p 127.0.0.1:27017:27017 \
  -e MONGO_INITDB_ROOT_USERNAME=root \
  -e MONGO_INITDB_ROOT_PASSWORD=example \
  -e MONGO_INITDB_DATABASE=medman \
  mongo:7
```

Wait for readiness:

```bash
until docker exec medman-test-mongo \
  mongosh --quiet --eval 'db.runCommand({ping:1}).ok' \
  -u root -p example --authenticationDatabase admin >/dev/null 2>&1; do
  sleep 1
done
```

## 2. Build and start the backend

```bash
cd "$(git rev-parse --show-toplevel)"
cargo build
MONGODB_URI='mongodb://root:example@127.0.0.1:27017/medman?authSource=admin' \
JWT_SECRET='test-secret' \
MISTRAL_API_KEY='unused-for-this-test' \
PORT=3000 \
./target/debug/medman-backend > /tmp/medman.log 2>&1 &
echo $! > /tmp/medman.pid
```

Wait for `/api/health` to respond:

```bash
until curl -sf http://127.0.0.1:3000/api/health >/dev/null; do sleep 1; done
```

## 3. Seed deterministic test data

We use two known `ObjectId`s so assertions are stable.

```bash
USER_A='000000000000000000000001'
USER_B='000000000000000000000002'

docker exec medman-test-mongo mongosh \
  -u root -p example --authenticationDatabase admin --quiet medman \
  --eval "
    db.medication_plans.deleteMany({});
    db.medication_plans.insertMany([
      {
        user_id: ObjectId('${USER_A}'),
        medication_name: 'Ritalin',
        plan_content: 'seed plan A1',
        created_at: new Date(),
        focus_areas: ['digestive', 'mood']
      },
      {
        user_id: ObjectId('${USER_A}'),
        medication_name: 'Ibuprofen',
        plan_content: 'seed plan A2',
        created_at: new Date(),
        focus_areas: ['pain']
      }
    ]);
  "
```

## 4. Test cases

For each case, record both HTTP status and body. All cases must pass.

### T1 — happy path: user with two plans

```bash
curl -s -o /tmp/t1.json -w '%{http_code}\n' \
  "http://127.0.0.1:3000/api/medication/plans?user_id=${USER_A}"
jq 'length, [.[].medication_name] | sort, .[0] | keys' /tmp/t1.json
```

Expected: status `200`, length `2`, names `["Ibuprofen","Ritalin"]`,
each item has key `id` (not `_id`), and `id` + `user_id` are 24-char
hex strings (not nested `{"$oid": …}` objects).

### T2 — happy path: user with no plans

```bash
curl -s -o /tmp/t2.json -w '%{http_code}\n' \
  "http://127.0.0.1:3000/api/medication/plans?user_id=${USER_B}"
cat /tmp/t2.json
```

Expected: status `200`, body `[]`.

### T3 — invalid ObjectId

```bash
curl -s -o /tmp/t3.json -w '%{http_code}\n' \
  "http://127.0.0.1:3000/api/medication/plans?user_id=not-an-oid"
cat /tmp/t3.json
```

Expected: status `400`, body contains `"error"` mentioning a BSON OID
parse failure.

### T4 — missing query param

```bash
curl -s -o /tmp/t4.json -w '%{http_code}\n' \
  "http://127.0.0.1:3000/api/medication/plans"
cat /tmp/t4.json
```

Expected: status `400` (rejected by Axum's `Query` extractor before
the handler runs).

### T5 — POST still works on the same path

Confirms wiring `post(generate_plan).get(get_plans)` did not break the
existing POST. We only check the path is reachable for POST; we do not
exercise Mistral here. A `400` from validation (missing/invalid body)
is acceptable; a `405 Method Not Allowed` is a regression.

```bash
curl -s -o /tmp/t5.json -w '%{http_code}\n' \
  -X POST -H 'content-type: application/json' -d '{}' \
  "http://127.0.0.1:3000/api/medication/plans"
```

Expected: status is **not** `405`. Any of `400`/`415`/`422`/`500` is
acceptable for this smoke check.

## 5. End-to-end smoke test via docker-compose

Verifies the `Dockerfile` and `docker-compose.yml` fixes by building
and running the backend service stack from scratch. Skip this section
if you only need to verify the API contract.

```bash
cd "$(git rev-parse --show-toplevel)"
docker compose up -d --build mongodb backend

# Wait for backend health (compose runs it on host port 3000).
until curl -sf http://127.0.0.1:3000/api/health >/dev/null; do sleep 2; done

# Empty list against a fresh DB:
curl -s -w '\nHTTP %{http_code}\n' \
  "http://127.0.0.1:3000/api/medication/plans?user_id=000000000000000000000099"
```

Expected: backend builds, `/api/health` reachable, last call returns
`200` with body `[]`.

Cleanup:

```bash
docker compose down -v
```

## 6. Cleanup (native-binary path)

```bash
kill "$(cat /tmp/medman.pid)" 2>/dev/null || true
docker rm -f medman-test-mongo >/dev/null
rm -f /tmp/medman.{pid,log} /tmp/t{1,2,3,4,5}.json
```

## Pass criteria

T1–T5 all match expectations above. If any fail, capture the response
body and `/tmp/medman.log` and report on the PR.

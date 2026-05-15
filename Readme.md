# Architecture Design Docs: test-project/src/main.rs
## Axum System For Reservaion

1. Tread-Safety & State
-> Arc<Mutex<AppState>>: Arc (shares data across async threads safely)

-> Synchronous lock: std::sync::Mutex (this is for preventing race conditions)

2. Handler Logic

step 1: Idempotency check : Rejects duplicate user ids 

step2 : capacity check : drops request with 409 CEonflic

step3 Mutation:  safely decrement cointer and increment user ids




# Section C: Data Persistence : test-project/src/data_percistence.sql
# Database – SQL – Medium
# Driver Delivery Performance Analysis
```sql
`id` (int, PK)

- `driver_id` (int)
- `delivery_status` (varchar: 'COMPLETED', 'FAILED', 'LATE')
- `attempt_timestamp` (datetime)
```
Filtering Conditions: 
-> the metrics must be apply strictly to year 2024 spt
-> Boundaries range: attempt_timestamp >= '2024-09-01' and < 2024-10-01
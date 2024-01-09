# Decisions of the project

## RAM usage optimisation

- Could've used `Vec<u8>` for **ASCII-only kv-pairs**, instead of just `String`, but went with Strings, because in most cases, the benefits of using String (**Unicode support, ease of use, safety**) overweigh the memory savings of using Vec<u8>

## Models

- Created `KvPair model` for extending the functionality later (**want to add timestamps(for leaderbords, for example)**)
- Shard model (same as kv_pair, just more clear naming and + ShardEndpoint)

## Networking

- `5000` port for broadcasting

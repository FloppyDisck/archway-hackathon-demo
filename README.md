# Hackathon Demo

Presentation can be found [here](https://docs.google.com/presentation/d/1uwfQB8kO6FlQfWWSUeZdmczm5xvEwgXzxVlJauwDZqo/edit?usp=sharing).

## Reproducing the steps
The demo is made to be run using two of the provided branches.
First go to `benchmark/baseline` and run `cargo bench`, and then do the same in `benchmark/optimized`, you will see a generated folder in `contracts/storage-demo/archway_test_tubes`.
Inside there should be a folder for each of the benchmarked Txs found in `contracts/storage-demo/benches/contract.rs`.

# Studied Behaviors

Running benchmarks on dApps can help developers identify future issues as usage and stored data grows.

## Storing a list of items

### Store Items
![results](https://github.com/FloppyDisck/archway-hackathon-demo/assets/34169809/917aaa07-c018-4f5d-ad76-3d32c5fca73e)

### Add Item
![results](https://github.com/FloppyDisck/archway-hackathon-demo/assets/34169809/c9c82fb3-5d8b-40d7-8dc0-faafbdedcc0f)

### Read Item
![results](https://github.com/FloppyDisck/archway-hackathon-demo/assets/34169809/7f289fdb-e956-4fbe-8f89-62c4c29c2051)

## Handling complex configs

### Set Admin
![results](https://github.com/FloppyDisck/archway-hackathon-demo/assets/34169809/3f9a5f05-f992-4f77-a45c-89e681fd55ce)

### Read Admin
![results](https://github.com/FloppyDisck/archway-hackathon-demo/assets/34169809/137f2750-1aa6-4a2e-9681-f90d84fcbe9d)

## Real case scenario

### ArchID query
![results](https://github.com/FloppyDisck/archway-hackathon-demo/assets/34169809/5bade973-1877-49e2-9e44-82b1d8f7ef6c)

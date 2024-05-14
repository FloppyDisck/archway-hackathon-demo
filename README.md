# Hackathon Demo

Presentation can be found [here](https://docs.google.com/presentation/d/1uwfQB8kO6FlQfWWSUeZdmczm5xvEwgXzxVlJauwDZqo/edit?usp=sharing).

## Reproducing the steps
The demo is made to be run using two of the provided branches.
First go to `benchmark/baseline` and run `cargo bench`, and then do the same in `benchmark/optimized`, you will see a generated folder in `contracts/storage-demo/archway_test_tubes`.
Inside there should be a folder for each of the benchmarked Txs found in `contracts/storage-demo/benches/contract.rs`.

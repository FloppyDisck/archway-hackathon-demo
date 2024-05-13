use archway_test_tube::test_tube::runner::*;
use archway_test_tube::test_tube::{Account, Module, SigningAccount, Wasm};
use archway_test_tube::{arch, harness_main, ArchwayApp, Bench, BenchSaveConfig, Console, Setup};
use cosmwasm_std::{Addr, Uint128};
use storage_demo::msg::{ExecuteMsg, InstantiateMsg};

fn app() -> (ArchwayApp, SigningAccount) {
    let app = ArchwayApp::default();
    let mut account = app.init_account(&vec![arch(10000)]).unwrap();

    (app, account)
}

// Initialize everything including the demo contract
fn demo_setup(
    app: &ArchwayApp,
    account: &SigningAccount,
    data_size: usize,
    console: &Console,
) -> String {
    console.bench_msg("Initializing Demo dApp".to_string());

    let wasm = Wasm::new(app);
    let wasm_byte_code = std::fs::read("../../artifacts/storage_demo.wasm").unwrap();
    let code_id = wasm
        .store_code(&wasm_byte_code, None, &account)
        .unwrap()
        .data
        .code_id;

    let contract_addr = wasm
        .instantiate(
            code_id,
            &InstantiateMsg {
                data_size: data_size as u64,
            },
            None,
            Some("demo"),
            &[],
            &account,
        )
        .unwrap()
        .data
        .address;

    contract_addr
}

fn store_iterations() -> Vec<usize> {
    let mut a = vec![1, 5];

    for (limit, sum) in vec![(50, 5), (100, 10), (500, 25), (2500, 100)] {
        while *a.last().unwrap() < limit {
            a.push(a.last().unwrap() + sum);
        }
    }

    a
}

fn set_bench(bench: &mut Bench) {
    bench.set_history(vec![BenchSaveConfig::package_version(4)])
}

fn store_items(bench: &mut Bench) {
    let mut group = bench.group("store_items");

    group.iter_bench(
        |count: &usize, console: &Console| {
            let (app, account) = app();

            let demo = demo_setup(&app, &account, 1, console);

            Setup::new(
                app,
                demo,
                account,
                vec![],
                ExecuteMsg::StoreItems {
                    count: *count as u64,
                },
            )
        },
        store_iterations(),
    );
}

fn add_item(bench: &mut Bench) {
    let mut group = bench.group("add_item");

    group.iter_bench(
        |count: &usize, console: &Console| {
            let (app, account) = app();

            let demo = demo_setup(&app, &account, 1, console);

            console.bench_msg("Seeding storage".to_string());
            let wasm = Wasm::new(&app);
            wasm.execute(
                &demo,
                &ExecuteMsg::StoreItems {
                    count: *count as u64,
                },
                &vec![],
                &account,
            )
            .unwrap();

            Setup::new(
                app,
                demo,
                account,
                vec![],
                ExecuteMsg::AddItem {
                    item: (*count as u64) + 1,
                },
            )
        },
        store_iterations(),
    );
}

fn read_item(bench: &mut Bench) {
    let mut group = bench.group("read_item");

    group.iter_bench(
        |count: &usize, console: &Console| {
            let (app, account) = app();

            let demo = demo_setup(&app, &account, 1, console);

            console.bench_msg("Seeding storage".to_string());
            let wasm = Wasm::new(&app);
            wasm.execute(
                &demo,
                &ExecuteMsg::StoreItems {
                    count: *count as u64,
                },
                &vec![],
                &account,
            )
            .unwrap();

            Setup::new(
                app,
                demo,
                account,
                vec![],
                ExecuteMsg::ReadItem {
                    name: (*count as u64 / 2).to_string(),
                },
            )
        },
        store_iterations(),
    );
}

fn store_number(bench: &mut Bench) {
    let mut group = bench.group("store_number");

    group.iter_bench(
        |count: &usize, console: &Console| {
            let (app, account) = app();

            let demo = demo_setup(&app, &account, 1, console);

            Setup::new(
                app,
                demo,
                account,
                vec![],
                ExecuteMsg::StoreNumber {
                    iter: *count as u64,
                    numb: Uint128::from(100u128),
                },
            )
        },
        store_iterations(),
    );
}

fn read_number(bench: &mut Bench) {
    let mut group = bench.group("read_number");

    group.iter_bench(
        |count: &usize, console: &Console| {
            let (app, account) = app();

            let demo = demo_setup(&app, &account, 1, console);

            console.bench_msg("Seeding storage".to_string());
            let wasm = Wasm::new(&app);
            wasm.execute(
                &demo,
                &ExecuteMsg::StoreNumber {
                    iter: *count as u64,
                    numb: Uint128::from(100u128),
                },
                &vec![],
                &account,
            )
            .unwrap();

            Setup::new(
                app,
                demo,
                account,
                vec![],
                ExecuteMsg::ReadNumber {
                    iter: *count as u64,
                },
            )
        },
        store_iterations(),
    );
}

fn set_admin(bench: &mut Bench) {
    let mut group = bench.group("set_admin");

    group.iter_bench(
        |count: &usize, console: &Console| {
            let (app, account) = app();

            let demo = demo_setup(&app, &account, *count, console);

            Setup::new(
                app,
                demo,
                account,
                vec![],
                ExecuteMsg::SetAdmin {
                    admin: Addr::unchecked("address"),
                },
            )
        },
        store_iterations(),
    );
}

fn get_admin(bench: &mut Bench) {
    let mut group = bench.group("get_admin");

    group.iter_bench(
        |count: &usize, console: &Console| {
            let (app, account) = app();

            let demo = demo_setup(&app, &account, *count, console);

            Setup::new(app, demo, account, vec![], ExecuteMsg::GetAdmin {})
        },
        store_iterations(),
    );
}

fn archid_setup(
    app: &ArchwayApp,
    admin: &SigningAccount,
    console: &Console,
    // Returns archid account addr
) -> (String, SigningAccount) {
    console.bench_msg("Initializing ArchId dApps".to_string());

    let wasm = Wasm::new(app);

    let wasm_byte_code = std::fs::read("../../external_artifacts/archid_registry.wasm").unwrap();
    let registry_id = wasm
        .store_code(&wasm_byte_code, None, &admin)
        .unwrap()
        .data
        .code_id;

    let wasm_byte_code = std::fs::read("../../external_artifacts/archid_token.wasm").unwrap();
    let token_id = wasm
        .store_code(&wasm_byte_code, None, &admin)
        .unwrap()
        .data
        .code_id;

    // Mock address gets generated to act as a stand-in while we initialize the token
    let mut mock = app.init_account(&vec![arch(100)]).unwrap();
    let mut wallet = app.init_account(&vec![arch(100)]).unwrap();

    let registry = wasm
        .instantiate(
            registry_id,
            &archid_registry::msg::InstantiateMsg {
                admin: Addr::unchecked(admin.address()),
                wallet: Addr::unchecked(wallet.address()),
                cw721: Addr::unchecked(mock.address()),
                base_cost: Uint128::from(1u64),
                base_expiration: 10000000,
            },
            None,
            Some("registry"),
            &[],
            &admin,
        )
        .unwrap()
        .data
        .address;

    let token = wasm
        .instantiate(
            token_id,
            &archid_token::InstantiateMsg {
                name: "TESTNFT".to_string(),
                symbol: "TSNFT".to_string(),
                minter: registry.clone(),
            },
            None,
            Some("token"),
            &[],
            &admin,
        )
        .unwrap()
        .data
        .address;

    // Update registry
    wasm.execute(
        &registry,
        &archid_registry::msg::ExecuteMsg::UpdateConfig {
            config: archid_registry::state::Config {
                admin: Addr::unchecked(admin.address()),
                wallet: Addr::unchecked(wallet.address()),
                cw721: Addr::unchecked(token.clone()),
                base_cost: Uint128::from(1u64),
                base_expiration: 10000000,
            },
        },
        &[],
        &admin,
    )
    .unwrap();

    let user = app.init_account(&vec![arch(10000)]).unwrap();

    (registry, user)
}

fn archid_iterations() -> Vec<usize> {
    let mut a = vec![1, 5];

    for (limit, sum) in vec![
        (50, 5),
        (100, 10),
        (500, 25),
        (2500, 100),
        (20000, 2500),
        (100000, 20000),
    ] {
        while *a.last().unwrap() < limit {
            a.push(a.last().unwrap() + sum);
        }
    }

    a
}

struct ArchIDState {
    registry: String,
    user: SigningAccount,
}

fn query_archid(bench: &mut Bench) {
    let mut group = bench.group("archid");

    group.stateful_iter_bench(
        |console: &Console| {
            let (app, account) = app();

            let demo = demo_setup(&app, &account, 1, console);
            let (registry, user) = archid_setup(&app, &account, console);
            let wasm = Wasm::new(&app);

            // Save registry
            wasm.execute(
                &demo,
                &ExecuteMsg::SetArchIdRegistry {
                    contract: registry.clone(),
                },
                &[],
                &account,
            )
            .unwrap();

            (
                Setup::new(
                    app,
                    demo,
                    account,
                    vec![],
                    ExecuteMsg::TestArchID {
                        addr: Addr::unchecked(user.address()),
                    },
                ),
                ArchIDState { registry, user },
            )
        },
        |last: Option<usize>, current: usize, console: &Console, state: &mut ArchIDState, setup: &mut Setup<ExecuteMsg>| {
            let wasm = Wasm::new(&setup.app);

            console.bench_msg(format!(
                "Generating {} ArchID domains",
                current
            ));
            for i in (last.unwrap_or(0) + 1)..=current {
                wasm.execute(
                    &state.registry,
                    &archid_registry::msg::ExecuteMsg::Register {
                        name: format!("{:07}", i),
                    },
                    &[arch(1)],
                    &state.user,
                )
                    .unwrap();
            }
        },
        archid_iterations(),
    );
}

harness_main!(
    set_bench,
    store_items,
    add_item,
    read_item,
    store_number,
    read_number,
    set_admin,
    get_admin,
    query_archid
);

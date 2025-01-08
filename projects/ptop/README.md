# Polkadot Top (PTop)

* Execution

```
cd projects/ptop;
cargo build;
cd ../../;
./target/debug/ptop;
```

* Docs

```
cd projects/ptop;
cargo build;
cargo doc --open;
```

## Proposed PTop Tool

* Run `ptop` to:
  * View agile coretime (i.e. cpu) processes (similar to the Linux ‘top’ command that shows the processes on cpu cores

* Query regions of the Rococo Coretime chain (in PJS state query: broker > regions)
  * Show core block start, core no., core mask
    * e.g.
      # raw storage
      55555555555555555555
      # decode as UTF8 / Unicode first
      UUUUUUUUUU
      # exactly a 50% interlaced 80bit mask b01010101010101010101010101010101010101010101010101010101010101010101010101010101.
  * Show cores currently available for a candidate parachain blocks
    * how many candidates parachain blocks
    * elastic scaling
      * prevent parachains from acquiring multiple cores per block if elastic scaling not support
      * accept multiple backed candidate parachain blocks in `paras_inherent`, if there are multiple cores for that candidate parachain
        * check whether multiple backed candidate parachain blocks are a descendant of the current parachain head using `process_candidates`, which checks the dependency chain and the inherent data that contains the parent_head_hash, and checks the PersistedValidationData (PVD) hash against the one in the candidate receipt similar to https://github.com/paritytech/polkadot-sdk/blob/342d7205734b776f0b5d2b141dd730c51dfb2374/polkadot/runtime/parachains/src/util.rs#L30, then put the candidates in a table based on their PVD hashes, then we just need to lookup the hash provided in the descriptor to find the parent.
        * `ParachainsInherentData` format uses `BackedCandidate` primitive, and `validator_indices` format could be changed so the last bits in the field identify the core (if there are multiple). E.g. let say we want to support up to 4 cores per para for now, we could use 2 bits here
        * references
          * https://github.com/paritytech/polkadot-sdk/issues/3131
    * check dependencies between the candidates parachain blocks already occupying the cores in the provisioner.
      * note: we don't have access to `parent_head_data` of persisted validation data (PVD) in the provisioner, so we could offload this to the prospective-parachains subsystem, and supply a `required_path` that is not neccesarily ordered so prospective-parachains will order it.
      * note: it is not necessary to check that the candidates parachain blocks we propose for backing also form a chain, since prospective-parachains only provide valid chains, and the runtime will also check this in `paras_inherent`
    * runtime policy option 2
      * what do we do with candidates parachain blocks that are not included within one relay chain block.
      * say we have candidate parachain blocks A->B->C that are backed at relay chain block number (RCB) and pending availability and 3 cores for this parachain. A and B candidate parachain blocks are included, and say there's also a valid candidate parachain block D that descends from B.
      * we start the onchain inclusion process for all cores of the para when any of the candidates blocks become available.
      * where C is still pending availability and we get the chance to back a new block.
      * we would back a new block that descends from C (being optimistic that C will get included), which could lead to having all other cores depend on the availability of C, which could stall progress until C gets included or timed out.
      * if only candidate parachain blocks A, B have become available at RCB + 1, then we clear candidate parachain block C's core (availability canceled/forced timeout) and only include A, B. collators will have to rebuild on top of B and that is fine
      * if only B or C is included, then yeah, the para has lost a relay chain slot but that's life
      * references
        * https://github.com/paritytech/polkadot-sdk/issues/3130
        * https://github.com/paritytech/polkadot-sdk/pull/3233
  * Show dynamic block durations
    * dynamic block durations for pallets by adding a generic `InstantProvider` for use on a per-pallet basis that could be configured to use local blocknumber, relay blocknumber, of some timestamp inherent https://github.com/paritytech/polkadot-sdk/issues/3268
      * on-demand coretime - each core may be occupied by a parachain id, `next_up_on_available` for another parachain id to be a candidate of that parent parachain id has its bitfields recorded on-chain and the next block producer calls the provisioner that requests from prospective parachains the next backable candidate, and if there is no candidate the core transitions to scheduled and the other parachain id gets to be a candidate on the next iteration
    * references
      * polkadot-sdk/polkadot/node/core/provisioner/src/lib.rs
      * https://github.com/paritytech/polkadot-sdk/issues/3141
  * Show lease of a parachain id on Coretime chain
  * Show migration status
    * onboard/register the broker chain that creates assignments to a core for each of all existing parachains for the migration to the coretime chain
    * references
      * migration logic https://github.com/paritytech/polkadot-sdk/blob/57ffbdbd0e30767e6072dcdaffbe701134a10674/polkadot/runtime/parachains/src/assigner_coretime/migration.rs#L88
      * https://github.com/paritytech/polkadot-sdk/issues/2578
  * Show swap status (whether a parachain team directly swapped their legacy parachain lease that they bid on for this bulk coretime model on Coretime chain as requested by relay chain before the runtime upgrade migration that cancels traditional auctions at Coretime launch since they are being phased out in favour of direct coretime sales, or otherwise if they do it too late they are refunded if the parachain lease had not started, or otherwise they just start a bulk coretime model from scratch or run a parachain with on-demand coretime purchases)
    * Note:
      * broker pallet is notified of any parachain slot swaps performed on the relay chain by registering an `OnSwap` hook/trait (moved from runtime/common/traits.rs to runtime/parachains to be accessible from broker pallet) for the the coretime pallet that sends an XCM message to the broker chain and invokes a new extrinsic `swap_leases` (alternatively invoked by root) which updates `Leases` storage item (which keeps the legacy parachain leases) in the broker pallet that is in sync with Leases in slots pallet, and `do_swap_leases` swaps all lease sharing a `TaskId`
      * reference:
        * https://github.com/paritytech/polkadot-sdk/pull/3714
  * Show coretime core auctions
  * Show runtime upgrade status (cancels traditional auctions)
  * Show shared cores
    * system chains (share a core round-robin
      * Asset Hub + Collectives
      * Bridge Hub + People
      * references
        * https://github.com/paritytech/polkadot-sdk/issues/2578
    * lease chains (Coretime)
    * on-demand coretime pools (Coretime)
  * Show coretime prices
    * number of initial cores is number on Kusama plus a couple for continency
    * number of initial cores on Polkadot
      * cores supported by Polkadot
        * 200+ cores - dec 2024
        * 120 cores - may 2024 - launch coretime
          * 62 cores legacy
          * 45 cores bulk (30 ideal)
            * 550 smaller projects on Polkadot launched on new Bulk Cores (multiple in 1)
            * 125 small projects from Tanssi (or it will just hop on 2-3 Cores)
          * 15 cores instantaneous
        * 90 cores - dec 2023
        * 52 cores - jul 2023
      * bulk settings
        * launch - may 2024
          * bulk_limit 15
          * bulk_target 10
    * initial bulk coretime price
      * initial price required so teams may forecast their costs, initially the price-per-core/month should be set to a very low price of $1k USD/month
      * sold in monthly allotments of a number of cores, where price adapts to previous sales. if the cores sold is greater than `bulk_target`, then price per core increases for the next sale, otherwise it decreases.
        * `bulk_limit` is the number of cores sold in each sale
        * `bulk_target` is the ideal number of cores we want as a community to be sold in each sale
      * existing parachains with existing slots (legacy cores) get the entire 2 year lease they acquired in auction and no further payment until legacy lease expires (through a reserved core)
    * lead-in price factor
      * proposed 7-day sale period, where price descends from a high number to a lower number (`sale_price`) in a linear, block-by-block basis, similar to a "dutch auction", where currently, according to RFC-1, this is a factor of 2, so if the `sale_price` is 1, the start price at the beginning of the 7-day lead-in period is 2.
      * initial lead-in factor should be increased to at least `8` to avoid front-running
    * on-demand minimum price
    * number of initial cores for sale
    * number of system on-demand cores
    * renewal factors
    * parachain upgrade prices (no longer free)
      * https://github.com/paritytech/polkadot-sdk/pull/2372
    * protection mechanisms status
      * to keep any single entity from buying up all the core time.
    * references
      * https://github.com/paritytech/polkadot-sdk/issues/2723
      * https://forum.polkadot.network/t/initial-coretime-pricing/5187/2

* TODO 
  * test on Zombienet
    * uses this swap test https://github.com/paritytech/polkadot-sdk/blob/75074952a859f90213ea25257b71ec2189dbcfc1/polkadot/runtime/common/src/integration_tests.rs#L854
  * `bot bench substrate-pallet --pallet=pallet_broker`
  * `bot bench cumulus-coretime --pallet=pallet_broker`
  * `bot bench cumulus-coretime --runtime coretime-westend --pallet=pallet_broker`
  * Parachains Team Board - https://github.com/orgs/paritytech/projects/119/views/5
  * Rococo Coretime chain - https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Frococo-coretime-rpc.polkadot.io#/accounts
  * Learn Agile Coretime Docs - https://github.com/w3f/polkadot-wiki/pull/5733/files
  * RegionX
    * Purchase Core Time - https://app.regionx.tech/purchase
  * Lastic
    * https://github.com/LasticXYZ
    * https://test.lastic.xyz/instacore

## Existing Monitoring Tools

* ytop - https://github.com/cjbassi/ytop
  * cpu, memory, temperatures, disk, network, and processes
* bottom - https://github.com/ClementTsang/bottom
  * cpu, memory, temperatures, disk, network, and processes
* zenith - https://github.com/bvaisvil/zenith
  * cpu, memory, disk, network, and processes
* bb - https://github.com/epilys/bb
  * cpu, processes
  * No CLI

* References:
  * https://www.wezm.net/v2/posts/2020/rust-top-alternatives/
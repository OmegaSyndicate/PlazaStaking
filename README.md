# DeffiPlaza Staking contract
 
Below are the transaction manifests needed to use the contract:

## instantiate
```
CALL_METHOD
  Address("<ACCOUNT>")
  "withdraw"
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Decimal("100")
;

TAKE_ALL_FROM_WORKTOP
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Bucket("tokens")
;

CALL_FUNCTION
  Address("package_tdx_2_1p5nepg5dm6ssr5kuqlyd0v4y4fgmp2tgn4m0d3yttr3pj7d4n4gycw")
  "ASTRLSTAKING"
  "new"
  Address("<OWNER_BADGE>")
  Bucket("tokens")
  Address("<DAPP_DEFINITION_ADDRESS>")
;
```

## add stake
```
CALL_METHOD
  Address("<ACCOUNT>")
  "withdraw"
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Decimal("100")
;

TAKE_ALL_FROM_WORKTOP
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Bucket("tokens")
;

CALL_METHOD
	Address("<STAKE_COMPONENT_ADDRESS>")
	"add_stake"
	Bucket("tokens")
;

CALL_METHOD
    Address("<ACCOUNT>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

## remove stake
```
CALL_METHOD
  Address("<ACCOUNT>>")
  "withdraw"
  Address("<POOL_UNIT_RESOURCE_ADDRESS>")
  Decimal("10")
;

TAKE_ALL_FROM_WORKTOP
  Address("<POOL_UNIT_RESOURCE_ADDRESS>")
  Bucket("tokens")
;

CALL_METHOD
	Address("<STAKE_COMPONENT_ADDRESS>")
	"remove_stake"
	Bucket("tokens")
;

CALL_METHOD
    Address("<ACCOUNT>")
    "deposit_batch"
    Expression("ENTIRE_WORKTOP")
;
```

## airdrop

To reward users for staking it is possible to deposit `RESOURCE_ADDRESS_TO_STAKE` without receiving new Pool Units in return, making the existing Pool Units hold more `RESOURCE_ADDRESS_TO_STAKE` and increasing their value.

```

CALL_METHOD
  Address("<ACCOUNT_HOLDING_OWNER_BADGE>")
    "create_proof_of_non_fungibles"
    Address("<OWNER_BADGE>")
    Array<NonFungibleLocalId>(
        NonFungibleLocalId("#1#")
    )
;

CALL_METHOD
  Address("<ACCOUNT>")
  "withdraw"
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Decimal("<AMOUNT>")
;

TAKE_ALL_FROM_WORKTOP
  Address("<RESOURCE_ADDRESS_TO_STAKE>")
  Bucket("tokens")
;

CALL_METHOD
	Address("<STAKE_COMPONENT_ADDRESS>")
	"airdrop"
	Bucket("tokens")
;
```
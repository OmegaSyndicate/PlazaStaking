use scrypto::prelude::*;

#[blueprint]
#[events(StakingCreatedEvent, AddStakeEvent, RemoveStakeEvent, DepositRewardsEvent)]
mod plazastaking {
    enable_package_royalties! {
        new => Free;
        add_stake => Usd(dec!(0.1));
        remove_stake => Usd(dec!(0.1));
        deposit_rewards => Usd(dec!(0.5));
    }

    // Setting the access rules
    enable_method_auth! {
        methods {
            add_stake => PUBLIC;
            remove_stake => PUBLIC;
            deposit_rewards => PUBLIC;
        }
    }

    struct PlazaStaking {
        pool: Global<OneResourcePool>,
    }

    impl PlazaStaking {
        pub fn new(
            owner_badge: ResourceAddress,
            tokens: Bucket,
            dapp_def_address: ComponentAddress,
			info_url: String,
			description: String
        ) -> Bucket {

            let token = tokens.resource_address();
            let token_manager = ResourceManager::from(token);

            // Set some default variables to use in the metadata later on
            let symbol = token_manager
                .get_metadata("symbol")
                .unwrap_or(Some("STAKE".to_owned()))
                .unwrap_or("STAKE".to_owned());
            let icon_url = token_manager
                .get_metadata("icon_url")
                .unwrap_or(Some("https://radix.defiplaza.net/assets/img/babylon/defiplaza-badge-dapp.png".to_owned()))
                .unwrap_or("https://radix.defiplaza.net/assets/img/babylon/defiplaza-badge-dapp.png".to_owned());

            // Allocate a new address of wrapper component we're about to
            // instantiate.
            let (address_reservation, component_address) =
                Runtime::allocate_component_address(PlazaStaking::blueprint_id());
            
            // We would like to create a one-resource pool, update its metadata,
            // and then change the owner to be the component. Thus, we will
            // first start the owner role being the package global caller, set
            // the metadata, and then switch it to be the component global
            // caller badge. This is because in the current context, even after
            // the instantiation, the global caller context is the global
            // package.
            let one_resource_pool = Blueprint::<OneResourcePool>::instantiate(
                OwnerRole::Updatable(rule!(require(
                    NonFungibleGlobalId::package_of_direct_caller_badge(
                        Runtime::package_address()
                    )
                ))),
                rule!(require(NonFungibleGlobalId::global_caller_badge(
                    component_address
                ))),
                token,
                None,
            );

            // Getting the address of the pool unit resource.
            let pool_unit_resource_manager = one_resource_pool
                .get_metadata::<_, GlobalAddress>("pool_unit")
                .ok()
                .and_then(|value| {
                    value.map(ResourceAddress::try_from).and_then(Result::ok)
                })
                .map(ResourceManager::from)
                .unwrap();

            // Setting metadata on the pool unit.
            let stoken_symbol = format!("s{}", symbol);
            let stoken_name = format!("Staked {}", symbol);
            let stoken_icon = Url::of(format!(
                "https://assets.defiplaza.net/lptokens/{}_base.png",
                Runtime::bech32_encode_address(token_manager.address())
            ));

            pool_unit_resource_manager.set_metadata("symbol", stoken_symbol);
            pool_unit_resource_manager.set_metadata("name", stoken_name);
            pool_unit_resource_manager.set_metadata("icon_url", stoken_icon);
            pool_unit_resource_manager.set_metadata("stake_token", Runtime::bech32_encode_address(token_manager.address()));

            // Setting the owner role of the one resource pool and the pool unit
            // to be the component caller badge instead of the package caller
            // badge.
            let owner_role = rule!(require(
                NonFungibleGlobalId::global_caller_badge(component_address)
            ));
            one_resource_pool.set_owner_role(owner_role.clone());
            pool_unit_resource_manager.set_owner_role(owner_role);

            // Instantiate the wrapper component
            let component = Self { 
                pool: one_resource_pool 
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Fixed(rule!(require(owner_badge))))
            .with_address(address_reservation)
            .metadata(metadata! {
                init {
                    "name" => format!("{} Staking", symbol), updatable;
                    "description" => description, updatable;
                    "info_url" => info_url, updatable;
                    "icon_url" => icon_url, updatable;
                    "tags" => vec!["Staking"], updatable;
                    "dapp_definition" => dapp_def_address, updatable;
                }
            })
            .globalize();

            // Emit event a new component was created
            Runtime::emit_event(StakingCreatedEvent{token});

            // Add the first stake
            let stokens = component.add_stake(tokens);

            // return pool units
            stokens
        }

         pub fn add_stake(&mut self, tokens: Bucket) -> Bucket {
            let amount = tokens.amount();
            let tokens = self.pool.contribute(tokens);

            Runtime::emit_event(AddStakeEvent{amount});

            tokens
        }

        pub fn remove_stake(&mut self, stokens: Bucket) -> Bucket {
            let amount = stokens.amount();
            let tokens = self.pool.redeem(stokens);

            Runtime::emit_event(RemoveStakeEvent{amount});

            tokens
        }
       
        pub fn deposit_rewards(&mut self, tokens: Bucket) {
            let amount = tokens.amount();
            self.pool.protected_deposit(tokens);
            
            Runtime::emit_event(DepositRewardsEvent{amount});
            
            return;
        }
    }
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct StakingCreatedEvent {
    pub token: ResourceAddress
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct AddStakeEvent {
    pub amount: Decimal
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct RemoveStakeEvent {
    pub amount: Decimal
}

#[derive(ScryptoSbor, ScryptoEvent)]
pub struct DepositRewardsEvent {
    pub amount: Decimal
}
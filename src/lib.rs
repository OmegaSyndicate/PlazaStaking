use scrypto::prelude::*;

#[blueprint]
#[events(StakingCreatedEvent, AddStakeEvent, RemoveStakeEvent, DepositRewardsEvent)]
mod plazastaking {
    enable_package_royalties! {
        new => Free;
        add_stake => Usd(dec!(0.1));
        remove_stake => Usd(dec!(0.1));
        deposit_rewards => Usd(dec!(0.5));
        set_meta => Free;
    }

    // Setting the access rules
    enable_method_auth! {
        methods {
            add_stake => PUBLIC;
            remove_stake => PUBLIC;
            deposit_rewards => PUBLIC;
            set_meta => PUBLIC;
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
            let symbol = token_manager
                .get_metadata("symbol")
                .unwrap_or(Some("STAKE".to_owned()))
                .unwrap_or("STAKE".to_owned());
            // let name = token_manager.get_metadata("name").unwrap_or(Some("Token staking".to_owned())).unwrap_or("Token staking".to_owned());
            let icon_url = token_manager
                .get_metadata("icon_url")
                .unwrap_or(Some("https://radix.defiplaza.net/assets/img/babylon/defiplaza-badge-dapp.png".to_owned()))
                .unwrap_or("https://radix.defiplaza.net/assets/img/babylon/defiplaza-badge-dapp.png".to_owned());

            let (address_reservation, component_address) =
                Runtime::allocate_component_address(PlazaStaking::blueprint_id());
            let global_component_caller_badge =
                NonFungibleGlobalId::global_caller_badge(component_address);

            // let owner_role = OwnerRole::Fixed(rule!(require(owner_badge)));

            let pool = Blueprint::<OneResourcePool>::instantiate(
                OwnerRole::Fixed(rule!(require(global_component_caller_badge.clone()))),
                rule!(require(global_component_caller_badge)),
                token,
                None,
            );

            let component = Self { pool }
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

            // let stoken_bucket = component.add_stake(tokens);

            // let stoken_symbol = format!("s{}", symbol);
            // let stoken_name = format!("Staked {}", symbol);
            // let stoken_icon = Url::of(format!(
            //     "https://assets.defiplaza.net/lptokens/{}_base.png",
            //     Runtime::bech32_encode_address(token_manager.address()))
            // );

            // let stoken_manager = ResourceManager::from(stoken_bucket.resource_address());
            // stoken_manager.set_metadata("symbol", stoken_symbol);
            // stoken_manager.set_metadata("name", stoken_name);
            // stoken_manager.set_metadata("icon_url", stoken_icon);

            // return stoken_bucket;

            Runtime::emit_event(StakingCreatedEvent{token});

            let stokens = component.add_stake(tokens);

            let stoken_manager = ResourceManager::from(stokens.resource_address());
            component.set_meta(token_manager, stoken_manager);

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

        pub fn set_meta(&mut self, token_manager: ResourceManager, stoken_manager: ResourceManager) {
            let symbol = token_manager
                .get_metadata("symbol")
                .unwrap_or(Some("STAKE".to_owned()))
                .unwrap_or("STAKE".to_owned());

            let stoken_symbol = format!("s{}", symbol);
            let stoken_name = format!("Staked {}", symbol);
            let stoken_icon = Url::of(format!(
                "https://assets.defiplaza.net/lptokens/{}_base.png",
                Runtime::bech32_encode_address(token_manager.address())
            ));

            stoken_manager.set_metadata("symbol", stoken_symbol);
            stoken_manager.set_metadata("name", stoken_name);
            stoken_manager.set_metadata("icon_url", stoken_icon);
            stoken_manager.set_metadata("stake_token", Runtime::bech32_encode_address(token_manager.address()));

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
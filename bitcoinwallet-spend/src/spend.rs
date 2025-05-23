// ---------------- [ File: bitcoinwallet-spend/src/spend.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/wallet/spend.h]

pub struct Output {

    tx:            *const WalletTx,

    /**
      | Index in tx->vout.
      |
      */
    i:             i32,

    /**
      | Depth in block chain.
      | 
      | If > 0: the tx is on chain and has this many
      | confirmations.
      | 
      | If = 0: the tx is waiting confirmation.
      | 
      | If < 0: a conflicting tx is on chain and
      | has this many confirmations.
      |
      */
    n_depth:       i32,

    /**
      | Pre-computed estimated size of this
      | output as a fully-signed input in a transaction.
      | Can be -1 if it could not be calculated
      |
      */
    n_input_bytes: i32,

    /**
      | Whether we have the private keys to spend
      | this output
      |
      */
    spendable:     bool,

    /**
      | Whether we know how to spend this output,
      | ignoring the lack of keys
      |
      */
    solvable:      bool,

    /**
      | Whether to use the maximum sized, 72 byte
      | signature when calculating the size of the
      | input spend. 
      |
      | This should only be set when watch-only
      | outputs are allowed
      |
      */
    use_max_sig:   bool,

    /**
      | Whether this output is considered safe to
      | spend. 
      |
      | Unconfirmed transactions from outside keys
      | and unconfirmed replacement transactions
      | are considered unsafe and will not be used
      | to fund new spending transactions.
      |
      */
    safe:          bool,
}

impl Output {

    pub fn new(
        wallet:         &Wallet,
        wtx:            &WalletTx,
        in_:            i32,
        n_depth_in:     i32,
        spendable_in:   bool,
        solvable_in:    bool,
        safe_in:        bool,
        use_max_sig_in: Option<bool>) -> Self {
        let use_max_sig_in: bool = use_max_sig_in.unwrap_or(false);
        todo!();
        /*


            tx = &wtx; i = iIn; nDepth = nDepthIn; fSpendable = fSpendableIn; fSolvable = fSolvableIn; fSafe = fSafeIn; nInputBytes = -1; use_max_sig = use_max_sig_in;
            // If known and signable by the given wallet, compute nInputBytes
            // Failure will keep this value -1
            if (fSpendable) {
                nInputBytes = GetTxSpendSize(wallet, wtx, i, use_max_sig);
            }
        */
    }
    
    #[inline] pub fn get_input_coin(&self) -> InputCoin {
        
        todo!();
        /*
            return CInputCoin(tx->tx, i, nInputBytes);
        */
    }
}

pub struct TxSize {
    vsize:  i64, // default = { -1 }
    weight: i64, // default = { -1 }
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/spend.cpp]

pub const OUTPUT_GROUP_MAX_ENTRIES: usize = 100;

/**
  | Get the marginal bytes if spending the
  | specified output from this transaction
  |
  */
pub fn get_tx_spend_size(
        wallet:      &Wallet,
        wtx:         &WalletTx,
        out:         u32,
        use_max_sig: Option<bool>) -> i32 {
    
    let use_max_sig: bool = use_max_sig.unwrap_or(false);

    todo!();
        /*
            return CalculateMaximumSignedInputSize(wtx.tx->vout[out], &wallet, use_max_sig);
        */
}

impl Output {
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return strprintf("COutput(%s, %d, %d) [%s]", tx->GetHash().ToString(), i, nDepth, FormatMoney(tx->tx->vout[i].nValue));
        */
    }
}

/**
  | Get the marginal bytes of spending the
  | specified output
  |
  */
pub fn calculate_maximum_signed_input_size_with_signing_provider(
        txout:       &TxOut,
        provider:    *const SigningProvider,
        use_max_sig: Option<bool>) -> i32 {

    let use_max_sig: bool = use_max_sig.unwrap_or(false);
    
    todo!();
        /*
            CMutableTransaction txn;
        txn.vin.push_back(CTxIn(OutPoint()));
        if (!provider || !DummySignInput(*provider, txn.vin[0], txout, use_max_sig)) {
            return -1;
        }
        return GetVirtualTransactionInputSize(txn.vin[0]);
        */
}

pub fn calculate_maximum_signed_input_size_with_wallet(
        txout:       &TxOut,
        wallet:      *const Wallet,
        use_max_sig: Option<bool>) -> i32 {

    let use_max_sig: bool = use_max_sig.unwrap_or(false);
    
    todo!();
        /*
            const std::unique_ptr<SigningProvider> provider = wallet->GetSolvingProvider(txout.scriptPubKey);
        return CalculateMaximumSignedInputSize(txout, provider.get(), use_max_sig);
        */
}

/**
  | Calculate the size of the transaction
  | assuming all signatures are max size
  | 
  | Use DummySignatureCreator, which
  | inserts 71 byte signatures everywhere.
  | 
  | -----------
  | @note
  | 
  | this requires that all inputs must be
  | in mapWallet (eg the tx should be AllInputsMine).
  |
  | txouts needs to be in the order of tx.vin
  |
  */
pub fn calculate_maximum_signed_tx_size_with_txouts(
        tx:           &Transaction,
        wallet:       *const Wallet,
        txouts:       &Vec<TxOut>,
        coin_control: Option<*const CoinControl>) -> TxSize {
    
    todo!();
        /*
            CMutableTransaction txNew(tx);
        if (!wallet->DummySignTx(txNew, txouts, coin_control)) {
            return TxSize{-1, -1};
        }
        CTransaction ctx(txNew);
        int64_t vsize = GetVirtualTransactionSize(ctx);
        int64_t weight = GetTransactionWeight(ctx);
        return TxSize{vsize, weight};
        */
}

#[EXCLUSIVE_LOCKS_REQUIRED(wallet->cs_wallet)]
pub fn calculate_maximum_signed_tx_size(
        tx:           &Transaction,
        wallet:       *const Wallet,
        coin_control: Option<*const CoinControl>) -> TxSize {

    todo!();
        /*
            std::vector<CTxOut> txouts;
        // Look up the inputs. The inputs are either in the wallet, or in coin_control.
        for (const CTxIn& input : tx.vin) {
            const auto mi = wallet->mapWallet.find(input.prevout.hash);
            // Can not estimate size without knowing the input details
            if (mi != wallet->mapWallet.end()) {
                assert(input.prevout.n < mi->second.tx->vout.size());
                txouts.emplace_back(mi->second.tx->vout.at(input.prevout.n));
            } else if (coin_control) {
                CTxOut txout;
                if (!coin_control->GetExternalOutput(input.prevout, txout)) {
                    return TxSize{-1, -1};
                }
                txouts.emplace_back(txout);
            } else {
                return TxSize{-1, -1};
            }
        }
        return CalculateMaximumSignedTxSize(tx, wallet, txouts, coin_control);
        */
}

/**
  | populate vCoins with vector of available
  | COutputs.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn available_coins(
        wallet:               &Wallet,
        coins:                &mut Vec<Output>,
        coin_control:         Option<*const CoinControl>,
        n_minimum_amount:     Option<Amount>,
        n_maximum_amount:     Option<Amount>,
        n_minimum_sum_amount: Option<Amount>,
        n_maximum_count:      Option<u64>) {
    
    let n_minimum_amount:                Amount = n_minimum_amount.unwrap_or(1);
    let n_maximum_amount:                Amount = n_maximum_amount.unwrap_or(MAX_MONEY);
    let n_minimum_sum_amount:            Amount = n_minimum_sum_amount.unwrap_or(MAX_MONEY);
    let n_maximum_count:                    u64 = n_maximum_count.unwrap_or(0);

    todo!();
        /*
            AssertLockHeld(wallet.cs_wallet);

        vCoins.clear();
        CAmount nTotal = 0;
        // Either the WALLET_FLAG_AVOID_REUSE flag is not set (in which case we always allow), or we default to avoiding, and only in the case where
        // a coin control object is provided, and has the avoid address reuse flag set to false, do we allow already used addresses
        bool allow_used_addresses = !wallet.IsWalletFlagSet(WALLET_FLAG_AVOID_REUSE) || (coinControl && !coinControl->m_avoid_address_reuse);
        const int min_depth = {coinControl ? coinControl->m_min_depth : DEFAULT_MIN_DEPTH};
        const int max_depth = {coinControl ? coinControl->m_max_depth : DEFAULT_MAX_DEPTH};
        const bool only_safe = {coinControl ? !coinControl->m_include_unsafe_inputs : true};

        std::set<uint256> trusted_parents;
        for (const auto& entry : wallet.mapWallet)
        {
            const uint256& wtxid = entry.first;
            const CWalletTx& wtx = entry.second;

            if (!wallet.chain().checkFinalTx(*wtx.tx)) {
                continue;
            }

            if (wallet.IsTxImmatureCoinBase(wtx))
                continue;

            int nDepth = wallet.GetTxDepthInMainChain(wtx);
            if (nDepth < 0)
                continue;

            // We should not consider coins which aren't at least in our mempool
            // It's possible for these to be conflicted via ancestors which we may never be able to detect
            if (nDepth == 0 && !wtx.InMempool())
                continue;

            bool safeTx = CachedTxIsTrusted(wallet, wtx, trusted_parents);

            // We should not consider coins from transactions that are replacing
            // other transactions.
            //
            // Example: There is a transaction A which is replaced by bumpfee
            // transaction B. In this case, we want to prevent creation of
            // a transaction B' which spends an output of B.
            //
            // Reason: If transaction A were initially confirmed, transactions B
            // and B' would no longer be valid, so the user would have to create
            // a new transaction C to replace B'. However, in the case of a
            // one-block reorg, transactions B' and C might BOTH be accepted,
            // when the user only wanted one of them. Specifically, there could
            // be a 1-block reorg away from the chain where transactions A and C
            // were accepted to another chain where B, B', and C were all
            // accepted.
            if (nDepth == 0 && wtx.mapValue.count("replaces_txid")) {
                safeTx = false;
            }

            // Similarly, we should not consider coins from transactions that
            // have been replaced. In the example above, we would want to prevent
            // creation of a transaction A' spending an output of A, because if
            // transaction B were initially confirmed, conflicting with A and
            // A', we wouldn't want to the user to create a transaction D
            // intending to replace A', but potentially resulting in a scenario
            // where A, A', and D could all be accepted (instead of just B and
            // D, or just A and A' like the user would want).
            if (nDepth == 0 && wtx.mapValue.count("replaced_by_txid")) {
                safeTx = false;
            }

            if (only_safe && !safeTx) {
                continue;
            }

            if (nDepth < min_depth || nDepth > max_depth) {
                continue;
            }

            for (unsigned int i = 0; i < wtx.tx->vout.size(); i++) {
                // Only consider selected coins if add_inputs is false
                if (coinControl && !coinControl->m_add_inputs && !coinControl->IsSelected(OutPoint(entry.first, i))) {
                    continue;
                }

                if (wtx.tx->vout[i].nValue < nMinimumAmount || wtx.tx->vout[i].nValue > nMaximumAmount)
                    continue;

                if (coinControl && coinControl->HasSelected() && !coinControl->fAllowOtherInputs && !coinControl->IsSelected(OutPoint(entry.first, i)))
                    continue;

                if (wallet.IsLockedCoin(entry.first, i))
                    continue;

                if (wallet.IsSpent(wtxid, i))
                    continue;

                isminetype mine = wallet.IsMine(wtx.tx->vout[i]);

                if (mine == ISMINE_NO) {
                    continue;
                }

                if (!allow_used_addresses && wallet.IsSpentKey(wtxid, i)) {
                    continue;
                }

                std::unique_ptr<SigningProvider> provider = wallet.GetSolvingProvider(wtx.tx->vout[i].scriptPubKey);

                bool solvable = provider ? IsSolvable(*provider, wtx.tx->vout[i].scriptPubKey) : false;
                bool spendable = ((mine & ISMINE_SPENDABLE) != ISMINE_NO) || (((mine & ISMINE_WATCH_ONLY) != ISMINE_NO) && (coinControl && coinControl->fAllowWatchOnly && solvable));

                vCoins.push_back(COutput(wallet, wtx, i, nDepth, spendable, solvable, safeTx, (coinControl && coinControl->fAllowWatchOnly)));

                // Checks the sum amount of all UTXO's.
                if (nMinimumSumAmount != MAX_MONEY) {
                    nTotal += wtx.tx->vout[i].nValue;

                    if (nTotal >= nMinimumSumAmount) {
                        return;
                    }
                }

                // Checks the maximum number of UTXO's.
                if (nMaximumCount > 0 && vCoins.size() >= nMaximumCount) {
                    return;
                }
            }
        }
        */
}

pub fn get_available_balance(
        wallet:       &Wallet,
        coin_control: Option<*const CoinControl>) -> Amount {

    todo!();
        /*
            LOCK(wallet.cs_wallet);

        CAmount balance = 0;
        std::vector<COutput> vCoins;
        AvailableCoins(wallet, vCoins, coinControl);
        for (const COutput& out : vCoins) {
            if (out.fSpendable) {
                balance += out.tx->tx->vout[out.i].nValue;
            }
        }
        return balance;
        */
}

/**
  | Find non-change parent output.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn find_non_change_parent_output<'a>(
        wallet: &Wallet,
        tx:     &Transaction,
        output: i32) -> &'a TxOut {
    
    todo!();
        /*
            AssertLockHeld(wallet.cs_wallet);
        const CTransaction* ptx = &tx;
        int n = output;
        while (OutputIsChange(wallet, ptx->vout[n]) && ptx->vin.size() > 0) {
            const OutPoint& prevout = ptx->vin[0].prevout;
            auto it = wallet.mapWallet.find(prevout.hash);
            if (it == wallet.mapWallet.end() || it->second.tx->vout.size() <= prevout.n ||
                !wallet.IsMine(it->second.tx->vout[prevout.n])) {
                break;
            }
            ptx = it->second.tx.get();
            n = prevout.n;
        }
        return ptx->vout[n];
        */
}

/**
  | Return list of available coins and locked
  | coins grouped by non-change output
  | address.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn list_coins(wallet: &Wallet) -> HashMap<TxDestination,Vec<Output>> {
    
    todo!();
        /*
            AssertLockHeld(wallet.cs_wallet);

        std::map<TxDestination, std::vector<COutput>> result;
        std::vector<COutput> availableCoins;

        AvailableCoins(wallet, availableCoins);

        for (const COutput& coin : availableCoins) {
            TxDestination address;
            if ((coin.fSpendable || (wallet.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS) && coin.fSolvable)) &&
                ExtractDestination(FindNonChangeParentOutput(wallet, *coin.tx->tx, coin.i).scriptPubKey, address)) {
                result[address].emplace_back(std::move(coin));
            }
        }

        std::vector<OutPoint> lockedCoins;
        wallet.ListLockedCoins(lockedCoins);
        // Include watch-only for LegacyScriptPubKeyMan wallets without private keys
        const bool include_watch_only = wallet.GetLegacyScriptPubKeyMan() && wallet.IsWalletFlagSet(WALLET_FLAG_DISABLE_PRIVATE_KEYS);
        const isminetype is_mine_filter = include_watch_only ? ISMINE_WATCH_ONLY : ISMINE_SPENDABLE;
        for (const OutPoint& output : lockedCoins) {
            auto it = wallet.mapWallet.find(output.hash);
            if (it != wallet.mapWallet.end()) {
                int depth = wallet.GetTxDepthInMainChain(it->second);
                if (depth >= 0 && output.n < it->second.tx->vout.size() &&
                    wallet.IsMine(it->second.tx->vout[output.n]) == is_mine_filter
                ) {
                    TxDestination address;
                    if (ExtractDestination(FindNonChangeParentOutput(wallet, *it->second.tx, output.n).scriptPubKey, address)) {
                        result[address].emplace_back(
                            wallet, it->second, output.n, depth, true /* spendable */, true /* solvable */, false /* safe */);
                    }
                }
            }
        }

        return result;
        */
}

pub fn group_outputs(
        wallet:          &Wallet,
        outputs:         &Vec<Output>,
        coin_sel_params: &CoinSelectionParams,
        filter:          &CoinEligibilityFilter,
        positive_only:   bool) -> Vec<OutputGroup> {
    
    todo!();
        /*
            std::vector<OutputGroup> groups_out;

        if (!coin_sel_params.m_avoid_partial_spends) {
            // Allowing partial spends  means no grouping. Each COutput gets its own OutputGroup.
            for (const COutput& output : outputs) {
                // Skip outputs we cannot spend
                if (!output.fSpendable) continue;

                size_t ancestors, descendants;
                wallet.chain().getTransactionAncestry(output.tx->GetHash(), ancestors, descendants);
                CInputCoin input_coin = output.GetInputCoin();

                // Make an OutputGroup containing just this output
                OutputGroup group{coin_sel_params};
                group.Insert(input_coin, output.nDepth, CachedTxIsFromMe(wallet, *output.tx, ISMINE_ALL), ancestors, descendants, positive_only);

                // Check the OutputGroup's eligibility. Only add the eligible ones.
                if (positive_only && group.GetSelectionAmount() <= 0) continue;
                if (group.m_outputs.size() > 0 && group.EligibleForSpending(filter)) groups_out.push_back(group);
            }
            return groups_out;
        }

        // We want to combine COutputs that have the same scriptPubKey into single OutputGroups
        // except when there are more than OUTPUT_GROUP_MAX_ENTRIES COutputs grouped in an OutputGroup.
        // To do this, we maintain a map where the key is the scriptPubKey and the value is a vector of OutputGroups.
        // For each COutput, we check if the scriptPubKey is in the map, and if it is, the COutput's CInputCoin is added
        // to the last OutputGroup in the vector for the scriptPubKey. When the last OutputGroup has
        // OUTPUT_GROUP_MAX_ENTRIES CInputCoins, a new OutputGroup is added to the end of the vector.
        std::map<CScript, std::vector<OutputGroup>> spk_to_groups_map;
        for (const auto& output : outputs) {
            // Skip outputs we cannot spend
            if (!output.fSpendable) continue;

            size_t ancestors, descendants;
            wallet.chain().getTransactionAncestry(output.tx->GetHash(), ancestors, descendants);
            CInputCoin input_coin = output.GetInputCoin();
            CScript spk = input_coin.txout.scriptPubKey;

            std::vector<OutputGroup>& groups = spk_to_groups_map[spk];

            if (groups.size() == 0) {
                // No OutputGroups for this scriptPubKey yet, add one
                groups.emplace_back(coin_sel_params);
            }

            // Get the last OutputGroup in the vector so that we can add the CInputCoin to it
            // A pointer is used here so that group can be reassigned later if it is full.
            OutputGroup* group = &groups.back();

            // Check if this OutputGroup is full. We limit to OUTPUT_GROUP_MAX_ENTRIES when using -avoidpartialspends
            // to avoid surprising users with very high fees.
            if (group->m_outputs.size() >= OUTPUT_GROUP_MAX_ENTRIES) {
                // The last output group is full, add a new group to the vector and use that group for the insertion
                groups.emplace_back(coin_sel_params);
                group = &groups.back();
            }

            // Add the input_coin to group
            group->Insert(input_coin, output.nDepth, CachedTxIsFromMe(wallet, *output.tx, ISMINE_ALL), ancestors, descendants, positive_only);
        }

        // Now we go through the entire map and pull out the OutputGroups
        for (const auto& spk_and_groups_pair: spk_to_groups_map) {
            const std::vector<OutputGroup>& groups_per_spk= spk_and_groups_pair.second;

            // Go through the vector backwards. This allows for the first item we deal with being the partial group.
            for (auto group_it = groups_per_spk.rbegin(); group_it != groups_per_spk.rend(); group_it++) {
                const OutputGroup& group = *group_it;

                // Don't include partial groups if there are full groups too and we don't want partial groups
                if (group_it == groups_per_spk.rbegin() && groups_per_spk.size() > 1 && !filter.m_include_partial_groups) {
                    continue;
                }

                // Check the OutputGroup's eligibility. Only add the eligible ones.
                if (positive_only && group.GetSelectionAmount() <= 0) continue;
                if (group.m_outputs.size() > 0 && group.EligibleForSpending(filter)) groups_out.push_back(group);
            }
        }

        return groups_out;
        */
}

/**
  | Shuffle and select coins until nTargetValue
  | is reached while avoiding small change;
  | This method is stochastic for some inputs
  | and upon completion the coin set and
  | corresponding actual target value
  | is assembled
  | 
  | -----------
  | @param[in] coins
  | 
  | Set of UTXOs to consider. These will
  | be categorized into
  | 
  | OutputGroups and filtered using eligibility_filter
  | before selecting coins.
  | ----------
  | @param[out] setCoinsRet
  | 
  | Populated with the coins selected if
  | successful.
  | ----------
  | @param[out] nValueRet
  | 
  | Used to return the total value of selected
  | coins.
  |
  */
pub fn attempt_selection(
        wallet:                &Wallet,
        n_target_value:        &Amount,
        eligibility_filter:    &CoinEligibilityFilter,
        coins:                 Vec<Output>,
        set_coins_ret:         &mut HashSet<InputCoin>,
        n_value_ret:           &mut Amount,
        coin_selection_params: &CoinSelectionParams) -> bool {
    
    todo!();
        /*
            setCoinsRet.clear();
        nValueRet = 0;
        // Vector of results for use with waste calculation
        // In order: calculated waste, selected inputs, selected input value (sum of input values)
        // TODO: Use a struct representing the selection result
        std::vector<std::tuple<CAmount, std::set<CInputCoin>, CAmount>> results;

        // Note that unlike KnapsackSolver, we do not include the fee for creating a change output as BnB will not create a change output.
        std::vector<OutputGroup> positive_groups = GroupOutputs(wallet, coins, coin_selection_params, eligibility_filter, true /* positive_only */);
        std::set<CInputCoin> bnb_coins;
        CAmount bnb_value;
        if (SelectCoinsBnB(positive_groups, nTargetValue, coin_selection_params.m_cost_of_change, bnb_coins, bnb_value)) {
            const auto waste = GetSelectionWaste(bnb_coins, /* cost of change */ CAmount(0), nTargetValue, !coin_selection_params.m_subtract_fee_outputs);
            results.emplace_back(std::make_tuple(waste, std::move(bnb_coins), bnb_value));
        }

        // The knapsack solver has some legacy behavior where it will spend dust outputs. We retain this behavior, so don't filter for positive only here.
        std::vector<OutputGroup> all_groups = GroupOutputs(wallet, coins, coin_selection_params, eligibility_filter, false /* positive_only */);
        // While nTargetValue includes the transaction fees for non-input things, it does not include the fee for creating a change output.
        // So we need to include that for KnapsackSolver as well, as we are expecting to create a change output.
        std::set<CInputCoin> knapsack_coins;
        CAmount knapsack_value;
        if (KnapsackSolver(nTargetValue + coin_selection_params.m_change_fee, all_groups, knapsack_coins, knapsack_value)) {
            const auto waste = GetSelectionWaste(knapsack_coins, coin_selection_params.m_cost_of_change, nTargetValue + coin_selection_params.m_change_fee, !coin_selection_params.m_subtract_fee_outputs);
            results.emplace_back(std::make_tuple(waste, std::move(knapsack_coins), knapsack_value));
        }

        // We include the minimum final change for SRD as we do want to avoid making really small change.
        // KnapsackSolver does not need this because it includes MIN_CHANGE internally.
        const CAmount srd_target = nTargetValue + coin_selection_params.m_change_fee + MIN_FINAL_CHANGE;
        auto srd_result = SelectCoinsSRD(positive_groups, srd_target);
        if (srd_result != std::nullopt) {
            const auto waste = GetSelectionWaste(srd_result->first, coin_selection_params.m_cost_of_change, srd_target, !coin_selection_params.m_subtract_fee_outputs);
            results.emplace_back(std::make_tuple(waste, std::move(srd_result->first), srd_result->second));
        }

        if (results.size() == 0) {
            // No solution found
            return false;
        }

        // Choose the result with the least waste
        // If the waste is the same, choose the one which spends more inputs.
        const auto& best_result = std::min_element(results.begin(), results.end(), [](const auto& a, const auto& b) {
            return std::get<0>(a) < std::get<0>(b) || (std::get<0>(a) == std::get<0>(b) && std::get<1>(a).size() > std::get<1>(b).size());
        });
        setCoinsRet = std::get<1>(*best_result);
        nValueRet = std::get<2>(*best_result);
        return true;
        */
}

/**
  | Select a set of coins such that nValueRet
  | >= nTargetValue and at least all coins
  | from coin_control are selected; never
  | select unconfirmed coins if they are
  | not ours
  | 
  | -----------
  | @param[out] setCoinsRet
  | 
  | Populated with inputs including pre-selected
  | inputs from coin_control and Coin Selection
  | if successful.
  | ----------
  | @param[out] nValueRet
  | 
  | Total value of selected coins including
  | pre-selected ones from coin_control
  | and Coin Selection if successful.
  |
  */
#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn select_coins(
        wallet:                &Wallet,
        available_coins:       &Vec<Output>,
        n_target_value:        &Amount,
        set_coins_ret:         &mut HashSet<InputCoin>,
        n_value_ret:           &mut Amount,
        coin_control:          &CoinControl,
        coin_selection_params: &mut CoinSelectionParams) -> bool {
    
    todo!();
        /*
            std::vector<COutput> vCoins(vAvailableCoins);
        CAmount value_to_select = nTargetValue;

        // coin control -> return all selected outputs (we want all selected to go into the transaction for sure)
        if (coin_control.HasSelected() && !coin_control.fAllowOtherInputs)
        {
            for (const COutput& out : vCoins)
            {
                if (!out.fSpendable)
                     continue;
                nValueRet += out.tx->tx->vout[out.i].nValue;
                setCoinsRet.insert(out.GetInputCoin());
            }
            return (nValueRet >= nTargetValue);
        }

        // calculate value from preset inputs and store them
        std::set<CInputCoin> setPresetCoins;
        CAmount nValueFromPresetInputs = 0;

        std::vector<OutPoint> vPresetInputs;
        coin_control.ListSelected(vPresetInputs);
        for (const OutPoint& outpoint : vPresetInputs) {
            int input_bytes = -1;
            CTxOut txout;
            std::map<uint256, CWalletTx>::const_iterator it = wallet.mapWallet.find(outpoint.hash);
            if (it != wallet.mapWallet.end()) {
                const CWalletTx& wtx = it->second;
                // Clearly invalid input, fail
                if (wtx.tx->vout.size() <= outpoint.n) {
                    return false;
                }
                input_bytes = GetTxSpendSize(wallet, wtx, outpoint.n, false);
                txout = wtx.tx->vout.at(outpoint.n);
            }
            if (input_bytes == -1) {
                // The input is external. We either did not find the tx in mapWallet, or we did but couldn't compute the input size with wallet data
                if (!coin_control.GetExternalOutput(outpoint, txout)) {
                    // Not ours, and we don't have solving data.
                    return false;
                }
                input_bytes = CalculateMaximumSignedInputSize(txout, &coin_control.m_external_provider, /* use_max_sig */ true);
            }

            CInputCoin coin(outpoint, txout, input_bytes);
            nValueFromPresetInputs += coin.txout.nValue;
            if (coin.m_input_bytes == -1) {
                return false; // Not solvable, can't estimate size for fee
            }
            coin.effective_value = coin.txout.nValue - coin_selection_params.m_effective_feerate.GetFee(coin.m_input_bytes);
            if (coin_selection_params.m_subtract_fee_outputs) {
                value_to_select -= coin.txout.nValue;
            } else {
                value_to_select -= coin.effective_value;
            }
            setPresetCoins.insert(coin);
        }

        // remove preset inputs from vCoins so that Coin Selection doesn't pick them.
        for (std::vector<COutput>::iterator it = vCoins.begin(); it != vCoins.end() && coin_control.HasSelected();)
        {
            if (setPresetCoins.count(it->GetInputCoin()))
                it = vCoins.erase(it);
            else
                ++it;
        }

        unsigned int limit_ancestor_count = 0;
        unsigned int limit_descendant_count = 0;
        wallet.chain().getPackageLimits(limit_ancestor_count, limit_descendant_count);
        const size_t max_ancestors = (size_t)std::max<int64_t>(1, limit_ancestor_count);
        const size_t max_descendants = (size_t)std::max<int64_t>(1, limit_descendant_count);
        const bool fRejectLongChains = gArgs.GetBoolArg("-walletrejectlongchains", DEFAULT_WALLET_REJECT_LONG_CHAINS);

        // form groups from remaining coins; note that preset coins will not
        // automatically have their associated (same address) coins included
        if (coin_control.m_avoid_partial_spends && vCoins.size() > OUTPUT_GROUP_MAX_ENTRIES) {
            // Cases where we have 101+ outputs all pointing to the same destination may result in
            // privacy leaks as they will potentially be deterministically sorted. We solve that by
            // explicitly shuffling the outputs before processing
            Shuffle(vCoins.begin(), vCoins.end(), FastRandomContext());
        }

        // Coin Selection attempts to select inputs from a pool of eligible UTXOs to fund the
        // transaction at a target feerate. If an attempt fails, more attempts may be made using a more
        // permissive CoinEligibilityFilter.
        const bool res = [&] {
            // Pre-selected inputs already cover the target amount.
            if (value_to_select <= 0) return true;

            // If possible, fund the transaction with confirmed UTXOs only. Prefer at least six
            // confirmations on outputs received from other wallets and only spend confirmed change.
            if (AttemptSelection(wallet, value_to_select, CoinEligibilityFilter(1, 6, 0), vCoins, setCoinsRet, nValueRet, coin_selection_params)) return true;
            if (AttemptSelection(wallet, value_to_select, CoinEligibilityFilter(1, 1, 0), vCoins, setCoinsRet, nValueRet, coin_selection_params)) return true;

            // Fall back to using zero confirmation change (but with as few ancestors in the mempool as
            // possible) if we cannot fund the transaction otherwise.
            if (wallet.m_spend_zero_conf_change) {
                if (AttemptSelection(wallet, value_to_select, CoinEligibilityFilter(0, 1, 2), vCoins, setCoinsRet, nValueRet, coin_selection_params)) return true;
                if (AttemptSelection(wallet, value_to_select, CoinEligibilityFilter(0, 1, std::min((size_t)4, max_ancestors/3), std::min((size_t)4, max_descendants/3)),
                                       vCoins, setCoinsRet, nValueRet, coin_selection_params)) {
                    return true;
                }
                if (AttemptSelection(wallet, value_to_select, CoinEligibilityFilter(0, 1, max_ancestors/2, max_descendants/2),
                                       vCoins, setCoinsRet, nValueRet, coin_selection_params)) {
                    return true;
                }
                // If partial groups are allowed, relax the requirement of spending OutputGroups (groups
                // of UTXOs sent to the same address, which are obviously controlled by a single wallet)
                // in their entirety.
                if (AttemptSelection(wallet, value_to_select, CoinEligibilityFilter(0, 1, max_ancestors-1, max_descendants-1, true /* include_partial_groups */),
                                       vCoins, setCoinsRet, nValueRet, coin_selection_params)) {
                    return true;
                }
                // Try with unsafe inputs if they are allowed. This may spend unconfirmed outputs
                // received from other wallets.
                if (coin_control.m_include_unsafe_inputs
                    && AttemptSelection(wallet, value_to_select,
                        CoinEligibilityFilter(0 /* conf_mine */, 0 /* conf_theirs */, max_ancestors-1, max_descendants-1, true /* include_partial_groups */),
                        vCoins, setCoinsRet, nValueRet, coin_selection_params)) {
                    return true;
                }
                // Try with unlimited ancestors/descendants. The transaction will still need to meet
                // mempool ancestor/descendant policy to be accepted to mempool and broadcasted, but
                // OutputGroups use heuristics that may overestimate ancestor/descendant counts.
                if (!fRejectLongChains && AttemptSelection(wallet, value_to_select,
                                          CoinEligibilityFilter(0, 1, std::numeric_limits<uint64_t>::max(), std::numeric_limits<uint64_t>::max(), true /* include_partial_groups */),
                                          vCoins, setCoinsRet, nValueRet, coin_selection_params)) {
                    return true;
                }
            }
            // Coin Selection failed.
            return false;
        }();

        // AttemptSelection clears setCoinsRet, so add the preset inputs from coin_control to the coinset
        util::insert(setCoinsRet, setPresetCoins);

        // add preset inputs to the total value selected
        nValueRet += nValueFromPresetInputs;

        return res;
        */
}

pub fn is_current_for_anti_fee_sniping<'a>(
        chain:      &'a mut dyn ChainInterface,
        block_hash: &u256) -> bool {
    
    todo!();
        /*
            if (chain.isInitialBlockDownload()) {
            return false;
        }
        constexpr int64_t MAX_ANTI_FEE_SNIPING_TIP_AGE = 8 * 60 * 60; // in seconds
        int64_t block_time;
        CHECK_NONFATAL(chain.findBlock(block_hash, FoundBlock().time(block_time)));
        if (block_time < (GetTime() - MAX_ANTI_FEE_SNIPING_TIP_AGE)) {
            return false;
        }
        return true;
        */
}

/**
  | Return a height-based locktime for
  | new transactions (uses the height of
  | the current chain tip unless we are not
  | synced with the current chain
  |
  */
pub fn get_locktime_for_new_transaction<'a>(
    chain:        &'a mut dyn ChainInterface,
    block_hash:   &u256,
    block_height: i32) -> u32 {
    
    todo!();
        /*
            uint32_t locktime;
        // Discourage fee sniping.
        //
        // For a large miner the value of the transactions in the best block and
        // the mempool can exceed the cost of deliberately attempting to mine two
        // blocks to orphan the current best block. By setting nLockTime such that
        // only the next block can include the transaction, we discourage this
        // practice as the height restricted and limited blocksize gives miners
        // considering fee sniping fewer options for pulling off this attack.
        //
        // A simple way to think about this is from the wallet's point of view we
        // always want the blockchain to move forward. By setting nLockTime this
        // way we're basically making the statement that we only want this
        // transaction to appear in the next block; we don't want to potentially
        // encourage reorgs by allowing transactions to appear at lower heights
        // than the next block in forks of the best chain.
        //
        // Of course, the subsidy is high enough, and transaction volume low
        // enough, that fee sniping isn't a problem yet, but by implementing a fix
        // now we ensure code won't be written that makes assumptions about
        // nLockTime that preclude a fix later.
        if (IsCurrentForAntiFeeSniping(chain, block_hash)) {
            locktime = block_height;

            // Secondly occasionally randomly pick a nLockTime even further back, so
            // that transactions that are delayed after signing for whatever reason,
            // e.g. high-latency mix networks and some CoinJoin implementations, have
            // better privacy.
            if (GetRandInt(10) == 0)
                locktime = std::max(0, (int)locktime - GetRandInt(100));
        } else {
            // If our chain is lagging behind, we can't discourage fee sniping nor help
            // the privacy of high-latency transactions. To avoid leaking a potentially
            // unique "nLockTime fingerprint", set nLockTime to a constant.
            locktime = 0;
        }
        assert(locktime < LOCKTIME_THRESHOLD);
        return locktime;
        */
}

#[EXCLUSIVE_LOCKS_REQUIRED(wallet.cs_wallet)]
pub fn create_transaction_internal(
    wallet:              &mut Wallet,
    vec_send:            &Vec<Recipient>,
    tx:                  &mut TransactionRef,
    n_fee_ret:           &mut Amount,
    n_change_pos_in_out: &mut i32,
    error:               &mut BilingualStr,
    coin_control:        &CoinControl,
    fee_calc_out:        &mut FeeCalculation,
    sign:                bool) -> bool {

    todo!();
        /*
            AssertLockHeld(wallet.cs_wallet);

        CMutableTransaction txNew; // The resulting transaction that we make
        txNew.nLockTime = GetLocktimeForNewTransaction(wallet.chain(), wallet.GetLastBlockHash(), wallet.GetLastBlockHeight());

        CoinSelectionParams coin_selection_params; // Parameters for coin selection, init with dummy
        coin_selection_params.m_avoid_partial_spends = coin_control.m_avoid_partial_spends;

        // Set the long term feerate estimate to the wallet's consolidate feerate
        coin_selection_params.m_long_term_feerate = wallet.m_consolidate_feerate;

        CAmount recipients_sum = 0;
        const OutputType change_type = wallet.TransactionChangeType(coin_control.m_change_type ? *coin_control.m_change_type : wallet.m_default_change_type, vecSend);
        ReserveDestination reservedest(&wallet, change_type);
        unsigned int outputs_to_subtract_fee_from = 0; // The number of outputs which we are subtracting the fee from
        for (const auto& recipient : vecSend) {
            recipients_sum += recipient.nAmount;

            if (recipient.fSubtractFeeFromAmount) {
                outputs_to_subtract_fee_from++;
                coin_selection_params.m_subtract_fee_outputs = true;
            }
        }

        // Create change script that will be used if we need change
        // TODO: pass in scriptChange instead of reservedest so
        // change transaction isn't always pay-to-bitcoin-address
        CScript scriptChange;

        // coin control: send change to custom address
        if (!std::get_if<CNoDestination>(&coin_control.destChange)) {
            scriptChange = GetScriptForDestination(coin_control.destChange);
        } else { // no coin control: send change to newly generated address
            // Note: We use a new key here to keep it from being obvious which side is the change.
            //  The drawback is that by not reusing a previous key, the change may be lost if a
            //  backup is restored, if the backup doesn't have the new private key for the change.
            //  If we reused the old key, it would be possible to add code to look for and
            //  rediscover unknown transactions that were written with keys of ours to recover
            //  post-backup change.

            // Reserve a new key pair from key pool. If it fails, provide a dummy
            // destination in case we don't need change.
            TxDestination dest;
            bilingual_str dest_err;
            if (!reservedest.GetReservedDestination(dest, true, dest_err)) {
                error = _("Transaction needs a change address, but we can't generate it.") + Untranslated(" ") + dest_err;
            }
            scriptChange = GetScriptForDestination(dest);
            // A valid destination implies a change script (and
            // vice-versa). An empty change script will abort later, if the
            // change keypool ran out, but change is required.
            CHECK_NONFATAL(IsValidDestination(dest) != scriptChange.empty());
        }
        CTxOut change_prototype_txout(0, scriptChange);
        coin_selection_params.change_output_size = GetSerializeSize(change_prototype_txout);

        // Get size of spending the change output
        int change_spend_size = CalculateMaximumSignedInputSize(change_prototype_txout, &wallet);
        // If the wallet doesn't know how to sign change output, assume p2sh-p2wpkh
        // as lower-bound to allow BnB to do it's thing
        if (change_spend_size == -1) {
            coin_selection_params.change_spend_size = DUMMY_NESTED_P2WPKH_INPUT_SIZE;
        } else {
            coin_selection_params.change_spend_size = (size_t)change_spend_size;
        }

        // Set discard feerate
        coin_selection_params.m_discard_feerate = GetDiscardRate(wallet);

        // Get the fee rate to use effective values in coin selection
        FeeCalculation feeCalc;
        coin_selection_params.m_effective_feerate = GetMinimumFeeRate(wallet, coin_control, &feeCalc);
        // Do not, ever, assume that it's fine to change the fee rate if the user has explicitly
        // provided one
        if (coin_control.m_feerate && coin_selection_params.m_effective_feerate > *coin_control.m_feerate) {
            error = strprintf(_("Fee rate (%s) is lower than the minimum fee rate setting (%s)"), coin_control.m_feerate->ToString(FeeEstimateMode::SAT_VB), coin_selection_params.m_effective_feerate.ToString(FeeEstimateMode::SAT_VB));
            return false;
        }
        if (feeCalc.reason == FeeReason::FALLBACK && !wallet.m_allow_fallback_fee) {
            // eventually allow a fallback fee
            error = _("Fee estimation failed. Fallbackfee is disabled. Wait a few blocks or enable -fallbackfee.");
            return false;
        }

        // Calculate the cost of change
        // Cost of change is the cost of creating the change output + cost of spending the change output in the future.
        // For creating the change output now, we use the effective feerate.
        // For spending the change output in the future, we use the discard feerate for now.
        // So cost of change = (change output size * effective feerate) + (size of spending change output * discard feerate)
        coin_selection_params.m_change_fee = coin_selection_params.m_effective_feerate.GetFee(coin_selection_params.change_output_size);
        coin_selection_params.m_cost_of_change = coin_selection_params.m_discard_feerate.GetFee(coin_selection_params.change_spend_size) + coin_selection_params.m_change_fee;

        // vouts to the payees
        if (!coin_selection_params.m_subtract_fee_outputs) {
            coin_selection_params.tx_noinputs_size = 11; // Static vsize overhead + outputs vsize. 4 nVersion, 4 nLocktime, 1 input count, 1 output count, 1 witness overhead (dummy, flag, stack size)
        }
        for (const auto& recipient : vecSend)
        {
            CTxOut txout(recipient.nAmount, recipient.scriptPubKey);

            // Include the fee cost for outputs.
            if (!coin_selection_params.m_subtract_fee_outputs) {
                coin_selection_params.tx_noinputs_size += ::GetSerializeSize(txout, PROTOCOL_VERSION);
            }

            if (IsDust(txout, wallet.chain().relayDustFee()))
            {
                error = _("Transaction amount too small");
                return false;
            }
            txNew.vout.push_back(txout);
        }

        // Include the fees for things that aren't inputs, excluding the change output
        const CAmount not_input_fees = coin_selection_params.m_effective_feerate.GetFee(coin_selection_params.tx_noinputs_size);
        CAmount selection_target = recipients_sum + not_input_fees;

        // Get available coins
        std::vector<COutput> vAvailableCoins;
        AvailableCoins(wallet, vAvailableCoins, &coin_control, 1, MAX_MONEY, MAX_MONEY, 0);

        // Choose coins to use
        CAmount inputs_sum = 0;
        std::set<CInputCoin> setCoins;
        if (!SelectCoins(wallet, vAvailableCoins, /* nTargetValue */ selection_target, setCoins, inputs_sum, coin_control, coin_selection_params))
        {
            error = _("Insufficient funds");
            return false;
        }

        // Always make a change output
        // We will reduce the fee from this change output later, and remove the output if it is too small.
        const CAmount change_and_fee = inputs_sum - recipients_sum;
        assert(change_and_fee >= 0);
        CTxOut newTxOut(change_and_fee, scriptChange);

        if (nChangePosInOut == -1)
        {
            // Insert change txn at random position:
            nChangePosInOut = GetRandInt(txNew.vout.size()+1);
        }
        else if ((unsigned int)nChangePosInOut > txNew.vout.size())
        {
            error = _("Change index out of range");
            return false;
        }

        assert(nChangePosInOut != -1);
        auto change_position = txNew.vout.insert(txNew.vout.begin() + nChangePosInOut, newTxOut);

        // Shuffle selected coins and fill in final vin
        std::vector<CInputCoin> selected_coins(setCoins.begin(), setCoins.end());
        Shuffle(selected_coins.begin(), selected_coins.end(), FastRandomContext());

        // Note how the sequence number is set to non-maxint so that
        // the nLockTime set above actually works.
        //
        // BIP125 defines opt-in RBF as any nSequence < maxint-1, so
        // we use the highest possible value in that range (maxint-2)
        // to avoid conflicting with other possible uses of nSequence,
        // and in the spirit of "smallest possible change from prior
        // behavior."
        const uint32_t nSequence = coin_control.m_signal_bip125_rbf.value_or(wallet.m_signal_rbf) ? MAX_BIP125_RBF_SEQUENCE : (CTxIn::SEQUENCE_FINAL - 1);
        for (const auto& coin : selected_coins) {
            txNew.vin.push_back(CTxIn(coin.outpoint, CScript(), nSequence));
        }

        // Calculate the transaction fee
        TxSize tx_sizes = CalculateMaximumSignedTxSize(CTransaction(txNew), &wallet, &coin_control);
        int nBytes = tx_sizes.vsize;
        if (nBytes == -1) {
            error = _("Missing solving data for estimating transaction size");
            return false;
        }
        nFeeRet = coin_selection_params.m_effective_feerate.GetFee(nBytes);

        // Subtract fee from the change output if not subtracting it from recipient outputs
        CAmount fee_needed = nFeeRet;
        if (!coin_selection_params.m_subtract_fee_outputs) {
            change_position->nValue -= fee_needed;
        }

        // We want to drop the change to fees if:
        // 1. The change output would be dust
        // 2. The change is within the (almost) exact match window, i.e. it is less than or equal to the cost of the change output (cost_of_change)
        CAmount change_amount = change_position->nValue;
        if (IsDust(*change_position, coin_selection_params.m_discard_feerate) || change_amount <= coin_selection_params.m_cost_of_change)
        {
            nChangePosInOut = -1;
            change_amount = 0;
            txNew.vout.erase(change_position);

            // Because we have dropped this change, the tx size and required fee will be different, so let's recalculate those
            tx_sizes = CalculateMaximumSignedTxSize(CTransaction(txNew), &wallet, &coin_control);
            nBytes = tx_sizes.vsize;
            fee_needed = coin_selection_params.m_effective_feerate.GetFee(nBytes);
        }

        // The only time that fee_needed should be less than the amount available for fees (in change_and_fee - change_amount) is when
        // we are subtracting the fee from the outputs. If this occurs at any other time, it is a bug.
        assert(coin_selection_params.m_subtract_fee_outputs || fee_needed <= change_and_fee - change_amount);

        // Update nFeeRet in case fee_needed changed due to dropping the change output
        if (fee_needed <= change_and_fee - change_amount) {
            nFeeRet = change_and_fee - change_amount;
        }

        // Reduce output values for subtractFeeFromAmount
        if (coin_selection_params.m_subtract_fee_outputs) {
            CAmount to_reduce = fee_needed + change_amount - change_and_fee;
            int i = 0;
            bool fFirst = true;
            for (const auto& recipient : vecSend)
            {
                if (i == nChangePosInOut) {
                    ++i;
                }
                CTxOut& txout = txNew.vout[i];

                if (recipient.fSubtractFeeFromAmount)
                {
                    txout.nValue -= to_reduce / outputs_to_subtract_fee_from; // Subtract fee equally from each selected recipient

                    if (fFirst) // first receiver pays the remainder not divisible by output count
                    {
                        fFirst = false;
                        txout.nValue -= to_reduce % outputs_to_subtract_fee_from;
                    }

                    // Error if this output is reduced to be below dust
                    if (IsDust(txout, wallet.chain().relayDustFee())) {
                        if (txout.nValue < 0) {
                            error = _("The transaction amount is too small to pay the fee");
                        } else {
                            error = _("The transaction amount is too small to send after the fee has been deducted");
                        }
                        return false;
                    }
                }
                ++i;
            }
            nFeeRet = fee_needed;
        }

        // Give up if change keypool ran out and change is required
        if (scriptChange.empty() && nChangePosInOut != -1) {
            return false;
        }

        if (sign && !wallet.SignTransaction(txNew)) {
            error = _("Signing transaction failed");
            return false;
        }

        // Return the constructed transaction data.
        tx = MakeTransactionRef(std::move(txNew));

        // Limit size
        if ((sign && GetTransactionWeight(*tx) > MAX_STANDARD_TX_WEIGHT) ||
            (!sign && tx_sizes.weight > MAX_STANDARD_TX_WEIGHT))
        {
            error = _("Transaction too large");
            return false;
        }

        if (nFeeRet > wallet.m_default_max_tx_fee) {
            error = TransactionErrorString(TransactionError::MAX_FEE_EXCEEDED);
            return false;
        }

        if (gArgs.GetBoolArg("-walletrejectlongchains", DEFAULT_WALLET_REJECT_LONG_CHAINS)) {
            // Lastly, ensure this tx will pass the mempool's chain limits
            if (!wallet.chain().checkChainLimits(tx)) {
                error = _("Transaction has too long of a mempool chain");
                return false;
            }
        }

        // Before we return success, we assume any change key will be used to prevent
        // accidental re-use.
        reservedest.KeepDestination();
        fee_calc_out = feeCalc;

        wallet.WalletLogPrintf("Fee Calculation: Fee:%d Bytes:%u Tgt:%d (requested %d) Reason:\"%s\" Decay %.5f: Estimation: (%g - %g) %.2f%% %.1f/(%.1f %d mem %.1f out) Fail: (%g - %g) %.2f%% %.1f/(%.1f %d mem %.1f out)\n",
                  nFeeRet, nBytes, feeCalc.returnedTarget, feeCalc.desiredTarget, StringForFeeReason(feeCalc.reason), feeCalc.est.decay,
                  feeCalc.est.pass.start, feeCalc.est.pass.end,
                  (feeCalc.est.pass.totalConfirmed + feeCalc.est.pass.inMempool + feeCalc.est.pass.leftMempool) > 0.0 ? 100 * feeCalc.est.pass.withinTarget / (feeCalc.est.pass.totalConfirmed + feeCalc.est.pass.inMempool + feeCalc.est.pass.leftMempool) : 0.0,
                  feeCalc.est.pass.withinTarget, feeCalc.est.pass.totalConfirmed, feeCalc.est.pass.inMempool, feeCalc.est.pass.leftMempool,
                  feeCalc.est.fail.start, feeCalc.est.fail.end,
                  (feeCalc.est.fail.totalConfirmed + feeCalc.est.fail.inMempool + feeCalc.est.fail.leftMempool) > 0.0 ? 100 * feeCalc.est.fail.withinTarget / (feeCalc.est.fail.totalConfirmed + feeCalc.est.fail.inMempool + feeCalc.est.fail.leftMempool) : 0.0,
                  feeCalc.est.fail.withinTarget, feeCalc.est.fail.totalConfirmed, feeCalc.est.fail.inMempool, feeCalc.est.fail.leftMempool);
        return true;
        */
}

/**
  | Create a new transaction paying the
  | recipients with a set of coins selected
  | by SelectCoins(); Also create the change
  | output, when needed
  | 
  | -----------
  | @note
  | 
  | passing nChangePosInOut as -1 will
  | result in setting a random position
  |
  */
pub fn create_transaction(
        wallet:              &mut Wallet,
        vec_send:            &Vec<Recipient>,
        tx:                  &mut TransactionRef,
        n_fee_ret:           &mut Amount,
        n_change_pos_in_out: &mut i32,
        error:               &mut BilingualStr,
        coin_control:        &CoinControl,
        fee_calc_out:        &mut FeeCalculation,
        sign:                Option<bool>) -> bool {

    let sign: bool = sign.unwrap_or(true);
    
    todo!();
        /*
            if (vecSend.empty()) {
            error = _("Transaction must have at least one recipient");
            return false;
        }

        if (std::any_of(vecSend.cbegin(), vecSend.cend(), [](const auto& recipient){ return recipient.nAmount < 0; })) {
            error = _("Transaction amounts must not be negative");
            return false;
        }

        LOCK(wallet.cs_wallet);

        int nChangePosIn = nChangePosInOut;
        Assert(!tx); // tx is an out-param. TODO change the return type from bool to tx (or nullptr)
        bool res = CreateTransactionInternal(wallet, vecSend, tx, nFeeRet, nChangePosInOut, error, coin_control, fee_calc_out, sign);
        // try with avoidpartialspends unless it's enabled already
        if (res && nFeeRet > 0 /* 0 means non-functional fee rate estimation */ && wallet.m_max_aps_fee > -1 && !coin_control.m_avoid_partial_spends) {
            CCoinControl tmp_cc = coin_control;
            tmp_cc.m_avoid_partial_spends = true;
            CAmount nFeeRet2;
            CTransactionRef tx2;
            int nChangePosInOut2 = nChangePosIn;
            bilingual_str error2; // fired and forgotten; if an error occurs, we discard the results
            if (CreateTransactionInternal(wallet, vecSend, tx2, nFeeRet2, nChangePosInOut2, error2, tmp_cc, fee_calc_out, sign)) {
                // if fee of this alternative one is within the range of the max fee, we use this one
                const bool use_aps = nFeeRet2 <= nFeeRet + wallet.m_max_aps_fee;
                wallet.WalletLogPrintf("Fee non-grouped = %lld, grouped = %lld, using %s\n", nFeeRet, nFeeRet2, use_aps ? "grouped" : "non-grouped");
                if (use_aps) {
                    tx = tx2;
                    nFeeRet = nFeeRet2;
                    nChangePosInOut = nChangePosInOut2;
                }
            }
        }
        return res;
        */
}

/**
  | Insert additional inputs into the transaction
  | by calling CreateTransaction();
  |
  */
pub fn fund_transaction(
        wallet:                        &mut Wallet,
        tx:                            &mut MutableTransaction,
        n_fee_ret:                     &mut Amount,
        n_change_pos_in_out:           &mut i32,
        error:                         &mut BilingualStr,
        lock_unspents:                 bool,
        set_subtract_fee_from_outputs: &HashSet<i32>,
        coin_control:                  CoinControl) -> bool {
    
    todo!();
        /*
            std::vector<CRecipient> vecSend;

        // Turn the txout set into a CRecipient vector.
        for (size_t idx = 0; idx < tx.vout.size(); idx++) {
            const CTxOut& txOut = tx.vout[idx];
            CRecipient recipient = {txOut.scriptPubKey, txOut.nValue, setSubtractFeeFromOutputs.count(idx) == 1};
            vecSend.push_back(recipient);
        }

        coinControl.fAllowOtherInputs = true;

        for (const CTxIn& txin : tx.vin) {
            coinControl.Select(txin.prevout);
        }

        // Acquire the locks to prevent races to the new locked unspents between the
        // CreateTransaction call and LockCoin calls (when lockUnspents is true).
        LOCK(wallet.cs_wallet);

        CTransactionRef tx_new;
        FeeCalculation fee_calc_out;
        if (!CreateTransaction(wallet, vecSend, tx_new, nFeeRet, nChangePosInOut, error, coinControl, fee_calc_out, false)) {
            return false;
        }

        if (nChangePosInOut != -1) {
            tx.vout.insert(tx.vout.begin() + nChangePosInOut, tx_new->vout[nChangePosInOut]);
        }

        // Copy output sizes from new transaction; they may have had the fee
        // subtracted from them.
        for (unsigned int idx = 0; idx < tx.vout.size(); idx++) {
            tx.vout[idx].nValue = tx_new->vout[idx].nValue;
        }

        // Add new txins while keeping original txin scriptSig/order.
        for (const CTxIn& txin : tx_new->vin) {
            if (!coinControl.IsSelected(txin.prevout)) {
                tx.vin.push_back(txin);

            }
            if (lockUnspents) {
                wallet.LockCoin(txin.prevout);
            }

        }

        return true;
        */
}

// ---------------- [ File: bitcoin-txmempool/tests/txvalidation.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/txvalidation_tests.cpp]

#[cfg(test)]
pub mod txvalidation_tests {

    /**
      | Ensure that the mempool won't accept
      | coinbase transactions.
      |
      */
    #[test] fn tx_mempool_reject_coinbase_test_chain_100setup() {
        todo!();
        /*
        
            CScript scriptPubKey = CScript() << ToByteVector(coinbaseKey.GetPubKey()) << OP_CHECKSIG;
            CMutableTransaction coinbaseTx;

            coinbaseTx.nVersion = 1;
            coinbaseTx.vin.resize(1);
            coinbaseTx.vout.resize(1);
            coinbaseTx.vin[0].scriptSig = CScript() << OP_11 << OP_EQUAL;
            coinbaseTx.vout[0].nValue = 1 * CENT;
            coinbaseTx.vout[0].scriptPubKey = scriptPubKey;

            BOOST_CHECK(CTransaction(coinbaseTx).IsCoinBase());

            LOCK(cs_main);

            unsigned int initialPoolSize = m_node.mempool->size();
            const MempoolAcceptResult result = AcceptToMemoryPool(m_node.chainman->ActiveChainstate(), *m_node.mempool, MakeTransactionRef(coinbaseTx),
                        true /* bypass_limits */);

            BOOST_CHECK(result.m_result_type == MempoolAcceptResult::ResultType::INVALID);

            // Check that the transaction hasn't been added to mempool.
            BOOST_CHECK_EQUAL(m_node.mempool->size(), initialPoolSize);

            // Check that the validation state reflects the unsuccessful attempt.
            BOOST_CHECK(result.m_state.IsInvalid());
            BOOST_CHECK_EQUAL(result.m_state.GetRejectReason(), "coinbase");
            BOOST_CHECK(result.m_state.GetResult() == TxValidationResult::TX_CONSENSUS);

        */
    }

    /**
      | Create placeholder transactions that
      | have no meaning.
      |
      */
    #[inline] pub fn create_placeholder_tx(
            num_inputs:  usize,
            num_outputs: usize) -> TransactionRef {
        
        todo!();
            /*
                CMutableTransaction mtx = CMutableTransaction();
                mtx.vin.resize(num_inputs);
                mtx.vout.resize(num_outputs);
                auto random_script = CScript() << ToByteVector(InsecureRand256()) << ToByteVector(InsecureRand256());
                for (size_t i{0}; i < num_inputs; ++i) {
                    mtx.vin[i].prevout.hash = InsecureRand256();
                    mtx.vin[i].prevout.n = 0;
                    mtx.vin[i].scriptSig = random_script;
                }
                for (size_t o{0}; o < num_outputs; ++o) {
                    mtx.vout[o].nValue = 1 * CENT;
                    mtx.vout[o].scriptPubKey = random_script;
                }
                return MakeTransactionRef(mtx);
            */
    }

    #[test] fn package_tests_test_chain_100setup() {
        todo!();
        /*
        
            LOCK(cs_main);
            unsigned int initialPoolSize = m_node.mempool->size();

            // Parent and Child Package
            CKey parent_key;
            parent_key.MakeNewKey(true);
            CScript parent_locking_script = GetScriptForDestination(PKHash(parent_key.GetPubKey()));
            auto mtx_parent = CreateValidMempoolTransaction(/* input_transaction */ m_coinbase_txns[0], /* vout */ 0,
                                                            /* input_height */ 0, /* input_signing_key */ coinbaseKey,
                                                            /* output_destination */ parent_locking_script,
                                                            /* output_amount */ CAmount(49 * COIN), /* submit */ false);
            CTransactionRef tx_parent = MakeTransactionRef(mtx_parent);

            CKey child_key;
            child_key.MakeNewKey(true);
            CScript child_locking_script = GetScriptForDestination(PKHash(child_key.GetPubKey()));
            auto mtx_child = CreateValidMempoolTransaction(/* input_transaction */ tx_parent, /* vout */ 0,
                                                           /* input_height */ 101, /* input_signing_key */ parent_key,
                                                           /* output_destination */ child_locking_script,
                                                           /* output_amount */ CAmount(48 * COIN), /* submit */ false);
            CTransactionRef tx_child = MakeTransactionRef(mtx_child);
            const auto result_parent_child = ProcessNewPackage(m_node.chainman->ActiveChainstate(), *m_node.mempool, {tx_parent, tx_child}, /* test_accept */ true);
            BOOST_CHECK_MESSAGE(result_parent_child.m_state.IsValid(),
                                "Package validation unexpectedly failed: " << result_parent_child.m_state.GetRejectReason());
            auto it_parent = result_parent_child.m_tx_results.find(tx_parent->GetWitnessHash());
            auto it_child = result_parent_child.m_tx_results.find(tx_child->GetWitnessHash());
            BOOST_CHECK(it_parent != result_parent_child.m_tx_results.end());
            BOOST_CHECK_MESSAGE(it_parent->second.m_state.IsValid(),
                                "Package validation unexpectedly failed: " << it_parent->second.m_state.GetRejectReason());
            BOOST_CHECK(it_child != result_parent_child.m_tx_results.end());
            BOOST_CHECK_MESSAGE(it_child->second.m_state.IsValid(),
                                "Package validation unexpectedly failed: " << it_child->second.m_state.GetRejectReason());

            // Packages can't have more than 25 transactions.
            Package package_too_many;
            package_too_many.reserve(MAX_PACKAGE_COUNT + 1);
            for (size_t i{0}; i < MAX_PACKAGE_COUNT + 1; ++i) {
                package_too_many.emplace_back(create_placeholder_tx(1, 1));
            }
            auto result_too_many = ProcessNewPackage(m_node.chainman->ActiveChainstate(), *m_node.mempool, package_too_many, /* test_accept */ true);
            BOOST_CHECK(result_too_many.m_state.IsInvalid());
            BOOST_CHECK_EQUAL(result_too_many.m_state.GetResult(), PackageValidationResult::PCKG_POLICY);
            BOOST_CHECK_EQUAL(result_too_many.m_state.GetRejectReason(), "package-too-many-transactions");

            // Packages can't have a total size of more than 101KvB.
            CTransactionRef large_ptx = create_placeholder_tx(150, 150);
            Package package_too_large;
            auto size_large = GetVirtualTransactionSize(*large_ptx);
            size_t total_size{0};
            while (total_size <= MAX_PACKAGE_SIZE * 1000) {
                package_too_large.push_back(large_ptx);
                total_size += size_large;
            }
            BOOST_CHECK(package_too_large.size() <= MAX_PACKAGE_COUNT);
            auto result_too_large = ProcessNewPackage(m_node.chainman->ActiveChainstate(), *m_node.mempool, package_too_large, /* test_accept */ true);
            BOOST_CHECK(result_too_large.m_state.IsInvalid());
            BOOST_CHECK_EQUAL(result_too_large.m_state.GetResult(), PackageValidationResult::PCKG_POLICY);
            BOOST_CHECK_EQUAL(result_too_large.m_state.GetRejectReason(), "package-too-large");

            // A single, giant transaction submitted through ProcessNewPackage fails on single tx policy.
            CTransactionRef giant_ptx = create_placeholder_tx(999, 999);
            BOOST_CHECK(GetVirtualTransactionSize(*giant_ptx) > MAX_PACKAGE_SIZE * 1000);
            auto result_single_large = ProcessNewPackage(m_node.chainman->ActiveChainstate(), *m_node.mempool, {giant_ptx}, /* test_accept */ true);
            BOOST_CHECK(result_single_large.m_state.IsInvalid());
            BOOST_CHECK_EQUAL(result_single_large.m_state.GetResult(), PackageValidationResult::PCKG_TX);
            BOOST_CHECK_EQUAL(result_single_large.m_state.GetRejectReason(), "transaction failed");
            auto it_giant_tx = result_single_large.m_tx_results.find(giant_ptx->GetWitnessHash());
            BOOST_CHECK(it_giant_tx != result_single_large.m_tx_results.end());
            BOOST_CHECK_EQUAL(it_giant_tx->second.m_state.GetRejectReason(), "tx-size");

            // Check that mempool size hasn't changed.
            BOOST_CHECK_EQUAL(m_node.mempool->size(), initialPoolSize);

        */
    }
}

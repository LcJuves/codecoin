// ---------------- [ File: bitcoin-blockfilter/tests/blockfilter_index.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/blockfilter_index_tests.cpp]

pub struct BuildChainTestingSetup {
    base: TestChain100Setup,
}

impl BuildChainTestingSetup {
    
    pub fn create_block(&mut self, 
        prev:           *const BlockIndex,
        txns:           &Vec<MutableTransaction>,
        script_pub_key: &Script) -> Block {
        
        todo!();
        /*
        
        */
    }
    
    pub fn build_chain(&mut self, 
        pindex:                  *const BlockIndex,
        coinbase_script_pub_key: &Script,
        length:                  usize,
        chain:                   &mut Vec<Arc<Block>>) -> bool {
        
        todo!();
        /*
        
        */
    }
}

pub fn check_filter_lookups(
        filter_index: &mut BlockFilterIndex,
        block_index:  *const BlockIndex,
        last_header:  &mut u256) -> bool {
    
    todo!();
        /*
            BlockFilter expected_filter;
        if (!ComputeFilter(filter_index.GetFilterType(), block_index, expected_filter)) {
            BOOST_ERROR("ComputeFilter failed on block " << block_index->nHeight);
            return false;
        }

        BlockFilter filter;
        uint256 filter_header;
        std::vector<BlockFilter> filters;
        std::vector<uint256> filter_hashes;

        BOOST_CHECK(filter_index.LookupFilter(block_index, filter));
        BOOST_CHECK(filter_index.LookupFilterHeader(block_index, filter_header));
        BOOST_CHECK(filter_index.LookupFilterRange(block_index->nHeight, block_index, filters));
        BOOST_CHECK(filter_index.LookupFilterHashRange(block_index->nHeight, block_index,
                                                       filter_hashes));

        BOOST_CHECK_EQUAL(filters.size(), 1U);
        BOOST_CHECK_EQUAL(filter_hashes.size(), 1U);

        BOOST_CHECK_EQUAL(filter.GetHash(), expected_filter.GetHash());
        BOOST_CHECK_EQUAL(filter_header, expected_filter.ComputeHeader(last_header));
        BOOST_CHECK_EQUAL(filters[0].GetHash(), expected_filter.GetHash());
        BOOST_CHECK_EQUAL(filter_hashes[0], expected_filter.GetHash());

        filters.clear();
        filter_hashes.clear();
        last_header = filter_header;
        return true;
        */
}

impl BuildChainTestingSetup {

    pub fn create_block(&mut self, 
        prev:           *const BlockIndex,
        txns:           &Vec<MutableTransaction>,
        script_pub_key: &Script) -> Block {
        
        todo!();
        /*
            const CChainParams& chainparams = Params();
        std::unique_ptr<CBlockTemplate> pblocktemplate = BlockAssembler(m_node.chainman->ActiveChainstate(), *m_node.mempool, chainparams).CreateNewBlock(scriptPubKey);
        CBlock& block = pblocktemplate->block;
        block.hashPrevBlock = prev->GetBlockHash();
        block.nTime = prev->nTime + 1;

        // Replace mempool-selected txns with just coinbase plus passed-in txns:
        block.vtx.resize(1);
        for (const CMutableTransaction& tx : txns) {
            block.vtx.push_back(MakeTransactionRef(tx));
        }
        // IncrementExtraNonce creates a valid coinbase and merkleRoot
        unsigned int extraNonce = 0;
        IncrementExtraNonce(&block, prev, extraNonce);

        while (!CheckProofOfWork(block.GetHash(), block.nBits, chainparams.GetConsensus())) ++block.nNonce;

        return block;
        */
    }
    
    pub fn build_chain(&mut self, 
        pindex:                  *const BlockIndex,
        coinbase_script_pub_key: &Script,
        length:                  usize,
        chain:                   &mut Vec<Arc<Block>>) -> bool {
        
        todo!();
        /*
            std::vector<CMutableTransaction> no_txns;

        chain.resize(length);
        for (auto& block : chain) {
            block = std::make_shared<CBlock>(CreateBlock(pindex, no_txns, coinbase_script_pub_key));
            CBlockHeader header = block->GetBlockHeader();

            BlockValidationState state;
            if (!Assert(m_node.chainman)->ProcessNewBlockHeaders({header}, state, Params(), &pindex)) {
                return false;
            }
        }

        return true;
        */
    }
}

#[test] fn blockfilter_index_initial_sync_build_chain_testing_setup() {
    todo!();
    /*
    
        BlockFilterIndex filter_index(BlockFilterType::BASIC, 1 << 20, true);

        uint256 last_header;

        // Filter should not be found in the index before it is started.
        {
            LOCK(cs_main);

            BlockFilter filter;
            uint256 filter_header;
            std::vector<BlockFilter> filters;
            std::vector<uint256> filter_hashes;

            for (const CBlockIndex* block_index = m_node.chainman->ActiveChain().Genesis();
                 block_index != nullptr;
                 block_index = m_node.chainman->ActiveChain().Next(block_index)) {
                BOOST_CHECK(!filter_index.LookupFilter(block_index, filter));
                BOOST_CHECK(!filter_index.LookupFilterHeader(block_index, filter_header));
                BOOST_CHECK(!filter_index.LookupFilterRange(block_index->nHeight, block_index, filters));
                BOOST_CHECK(!filter_index.LookupFilterHashRange(block_index->nHeight, block_index,
                                                                filter_hashes));
            }
        }

        // BlockUntilSyncedToCurrentChain should return false before index is started.
        BOOST_CHECK(!filter_index.BlockUntilSyncedToCurrentChain());

        BOOST_REQUIRE(filter_index.Start(m_node.chainman->ActiveChainstate()));

        // Allow filter index to catch up with the block index.
        constexpr int64_t timeout_ms = 10 * 1000;
        int64_t time_start = GetTimeMillis();
        while (!filter_index.BlockUntilSyncedToCurrentChain()) {
            BOOST_REQUIRE(time_start + timeout_ms > GetTimeMillis());
            UninterruptibleSleep(std::chrono::milliseconds{100});
        }

        // Check that filter index has all blocks that were in the chain before it started.
        {
            LOCK(cs_main);
            const CBlockIndex* block_index;
            for (block_index = m_node.chainman->ActiveChain().Genesis();
                 block_index != nullptr;
                 block_index = m_node.chainman->ActiveChain().Next(block_index)) {
                CheckFilterLookups(filter_index, block_index, last_header);
            }
        }

        // Create two forks.
        const CBlockIndex* tip;
        {
            LOCK(cs_main);
            tip = m_node.chainman->ActiveChain().Tip();
        }
        CKey coinbase_key_A, coinbase_key_B;
        coinbase_key_A.MakeNewKey(true);
        coinbase_key_B.MakeNewKey(true);
        CScript coinbase_script_pub_key_A = GetScriptForDestination(PKHash(coinbase_key_A.GetPubKey()));
        CScript coinbase_script_pub_key_B = GetScriptForDestination(PKHash(coinbase_key_B.GetPubKey()));
        std::vector<std::shared_ptr<CBlock>> chainA, chainB;
        BOOST_REQUIRE(BuildChain(tip, coinbase_script_pub_key_A, 10, chainA));
        BOOST_REQUIRE(BuildChain(tip, coinbase_script_pub_key_B, 10, chainB));

        // Check that new blocks on chain A get indexed.
        uint256 chainA_last_header = last_header;
        for (size_t i = 0; i < 2; i++) {
            const auto& block = chainA[i];
            BOOST_REQUIRE(Assert(m_node.chainman)->ProcessNewBlock(Params(), block, true, nullptr));
        }
        for (size_t i = 0; i < 2; i++) {
            const auto& block = chainA[i];
            const CBlockIndex* block_index;
            {
                LOCK(cs_main);
                block_index = m_node.chainman->m_blockman.LookupBlockIndex(block->GetHash());
            }

            BOOST_CHECK(filter_index.BlockUntilSyncedToCurrentChain());
            CheckFilterLookups(filter_index, block_index, chainA_last_header);
        }

        // Reorg to chain B.
        uint256 chainB_last_header = last_header;
        for (size_t i = 0; i < 3; i++) {
            const auto& block = chainB[i];
            BOOST_REQUIRE(Assert(m_node.chainman)->ProcessNewBlock(Params(), block, true, nullptr));
        }
        for (size_t i = 0; i < 3; i++) {
            const auto& block = chainB[i];
            const CBlockIndex* block_index;
            {
                LOCK(cs_main);
                block_index = m_node.chainman->m_blockman.LookupBlockIndex(block->GetHash());
            }

            BOOST_CHECK(filter_index.BlockUntilSyncedToCurrentChain());
            CheckFilterLookups(filter_index, block_index, chainB_last_header);
        }

        // Check that filters for stale blocks on A can be retrieved.
        chainA_last_header = last_header;
        for (size_t i = 0; i < 2; i++) {
            const auto& block = chainA[i];
            const CBlockIndex* block_index;
            {
                LOCK(cs_main);
                block_index = m_node.chainman->m_blockman.LookupBlockIndex(block->GetHash());
            }

            BOOST_CHECK(filter_index.BlockUntilSyncedToCurrentChain());
            CheckFilterLookups(filter_index, block_index, chainA_last_header);
        }

        // Reorg back to chain A.
         for (size_t i = 2; i < 4; i++) {
             const auto& block = chainA[i];
             BOOST_REQUIRE(Assert(m_node.chainman)->ProcessNewBlock(Params(), block, true, nullptr));
         }

         // Check that chain A and B blocks can be retrieved.
         chainA_last_header = last_header;
         chainB_last_header = last_header;
         for (size_t i = 0; i < 3; i++) {
             const CBlockIndex* block_index;

             {
                 LOCK(cs_main);
                 block_index = m_node.chainman->m_blockman.LookupBlockIndex(chainA[i]->GetHash());
             }
             BOOST_CHECK(filter_index.BlockUntilSyncedToCurrentChain());
             CheckFilterLookups(filter_index, block_index, chainA_last_header);

             {
                 LOCK(cs_main);
                 block_index = m_node.chainman->m_blockman.LookupBlockIndex(chainB[i]->GetHash());
             }
             BOOST_CHECK(filter_index.BlockUntilSyncedToCurrentChain());
             CheckFilterLookups(filter_index, block_index, chainB_last_header);
         }

        // Test lookups for a range of filters/hashes.
        std::vector<BlockFilter> filters;
        std::vector<uint256> filter_hashes;

        {
            LOCK(cs_main);
            tip = m_node.chainman->ActiveChain().Tip();
        }
        BOOST_CHECK(filter_index.LookupFilterRange(0, tip, filters));
        BOOST_CHECK(filter_index.LookupFilterHashRange(0, tip, filter_hashes));

        assert(tip->nHeight >= 0);
        BOOST_CHECK_EQUAL(filters.size(), tip->nHeight + 1U);
        BOOST_CHECK_EQUAL(filter_hashes.size(), tip->nHeight + 1U);

        filters.clear();
        filter_hashes.clear();

        filter_index.Interrupt();
        filter_index.Stop();

    */
}

#[test] fn blockfilter_index_init_destroy_basic_testing_setup() {
    todo!();
    /*
    
        BlockFilterIndex* filter_index;

        filter_index = GetBlockFilterIndex(BlockFilterType::BASIC);
        BOOST_CHECK(filter_index == nullptr);

        BOOST_CHECK(InitBlockFilterIndex(BlockFilterType::BASIC, 1 << 20, true, false));

        filter_index = GetBlockFilterIndex(BlockFilterType::BASIC);
        BOOST_CHECK(filter_index != nullptr);
        BOOST_CHECK(filter_index->GetFilterType() == BlockFilterType::BASIC);

        // Initialize returns false if index already exists.
        BOOST_CHECK(!InitBlockFilterIndex(BlockFilterType::BASIC, 1 << 20, true, false));

        int iter_count = 0;
        ForEachBlockFilterIndex([&iter_count](BlockFilterIndex& _index) { iter_count++; });
        BOOST_CHECK_EQUAL(iter_count, 1);

        BOOST_CHECK(DestroyBlockFilterIndex(BlockFilterType::BASIC));

        // Destroy returns false because index was already destroyed.
        BOOST_CHECK(!DestroyBlockFilterIndex(BlockFilterType::BASIC));

        filter_index = GetBlockFilterIndex(BlockFilterType::BASIC);
        BOOST_CHECK(filter_index == nullptr);

        // Reinitialize index.
        BOOST_CHECK(InitBlockFilterIndex(BlockFilterType::BASIC, 1 << 20, true, false));

        DestroyAllBlockFilterIndexes();

        filter_index = GetBlockFilterIndex(BlockFilterType::BASIC);
        BOOST_CHECK(filter_index == nullptr);

    */
}

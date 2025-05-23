// ---------------- [ File: bitcoin-bench/src/bench_checkqueue.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/checkqueue.cpp]

pub const batches:          usize = 101;
pub const batch_size:       usize = 30;
pub const prevector_size:   i32 = 28;
pub const queue_batch_size: u32 = 128;

/**
  | This Benchmark tests the CheckQueue with
  | a slightly realistic workload, where checks all
  | contain a prevector that is indirect 50% of the
  | time and there is a little bit of work done
  | between calls to Add.
  */
#[bench] fn check_queue_speed_prevector_job(b: &mut Bencher)  {
    
    todo!();
        /*
        // We shouldn't ever be running with the checkqueue on a single core machine.
        if (GetNumCores() <= 1) return;

        const ECCVerifyHandle verify_handle;
        ECC_Start();

        struct PrevectorJob {
            prevector<PREVECTOR_SIZE, uint8_t> p;
            PrevectorJob(){
            }
            explicit PrevectorJob(FastRandomContext& insecure_rand){
                p.resize(insecure_rand.randrange(PREVECTOR_SIZE*2));
            }
            bool operator()()
            {
                return true;
            }
            c_void swap(PrevectorJob& x){p.swap(x.p);};
        };
        CCheckQueue<PrevectorJob> queue {QUEUE_BATCH_SIZE};
        // The main thread should be counted to prevent thread oversubscription, and
        // to decrease the variance of benchmark results.
        queue.StartWorkerThreads(GetNumCores() - 1);

        // create all the data once, then submit copies in the benchmark.
        FastRandomContext insecure_rand(true);
        std::vector<std::vector<PrevectorJob>> vBatches(BATCHES);
        for (auto& vChecks : vBatches) {
            vChecks.reserve(BATCH_SIZE);
            for (size_t x = 0; x < BATCH_SIZE; ++x)
                vChecks.emplace_back(insecure_rand);
        }

        bench.minEpochIterations(10).batch(BATCH_SIZE * BATCHES).unit("job").run([&] {
            // Make insecure_rand here so that each iteration is identical.
            CCheckQueueControl<PrevectorJob> control(&queue);
            for (auto vChecks : vBatches) {
                control.Add(vChecks);
            }
            // control waits for completion by RAII, but
            // it is done explicitly here for clarity
            control.Wait();
        });
        queue.StopWorkerThreads();
        ECC_Stop();
        */
}

// ---------------- [ File: bitcoin-sha3/src/sha3.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha3.h]

#[derive(Default)]
pub struct SHA3_256 {
    state:   [u64; 25], // default = {0}
    buffer:  Sha3_256Buffer,
    bufsize: u32, // default = 0
    pos:     u32, // default = 0
}

pub type Sha3_256Buffer = [u8; 8];

/**
  | Sponge rate in bits.
  |
  */
pub const SHA3_256_RATE_BITS: usize = 1088;

/**
  | Sponge rate expressed as a multiple
  | of the buffer size.
  |
  */
pub const SHA3_256_RATE_BUFFERS: usize = SHA3_256_RATE_BITS / (8 * size_of::<Sha3_256Buffer>());

/**
  | error msg: "Rate must be a multiple of
  | 8 bytes"
  |
  */
const_assert!{
    SHA3_256_RATE_BITS % (8 * size_of::<Sha3_256Buffer>()) == 0
}

pub const SHA3_256_OUTPUT_SIZE: usize = 32;

impl SHA3_256 {

    pub fn write(&mut self, data: &[u8]) -> &mut SHA3_256 {
        
        todo!();
        /*
            if (m_bufsize && m_bufsize + data.size() >= sizeof(m_buffer)) {
            // Fill the buffer and process it.
            std::copy(data.begin(), data.begin() + sizeof(m_buffer) - m_bufsize, m_buffer + m_bufsize);
            data = data.subspan(sizeof(m_buffer) - m_bufsize);
            m_state[m_pos++] ^= ReadLE64(m_buffer);
            m_bufsize = 0;
            if (m_pos == RATE_BUFFERS) {
                KeccakF(m_state);
                m_pos = 0;
            }
        }
        while (data.size() >= sizeof(m_buffer)) {
            // Process chunks directly from the buffer.
            m_state[m_pos++] ^= ReadLE64(data.data());
            data = data.subspan(8);
            if (m_pos == RATE_BUFFERS) {
                KeccakF(m_state);
                m_pos = 0;
            }
        }
        if (data.size()) {
            // Keep the remainder in the buffer.
            std::copy(data.begin(), data.end(), m_buffer + m_bufsize);
            m_bufsize += data.size();
        }
        return *this;
        */
    }
    
    pub fn finalize(&mut self, output: &[u8]) -> &mut SHA3_256 {
        
        todo!();
        /*
            assert(output.size() == OUTPUT_SIZE);
        std::fill(m_buffer + m_bufsize, m_buffer + sizeof(m_buffer), 0);
        m_buffer[m_bufsize] ^= 0x06;
        m_state[m_pos] ^= ReadLE64(m_buffer);
        m_state[RATE_BUFFERS - 1] ^= 0x8000000000000000;
        KeccakF(m_state);
        for (unsigned i = 0; i < 4; ++i) {
            WriteLE64(output.data() + 8 * i, m_state[i]);
        }
        return *this;
        */
    }
    
    pub fn reset(&mut self) -> &mut SHA3_256 {
        
        todo!();
        /*
            m_bufsize = 0;
        m_pos = 0;
        std::fill(std::begin(m_state), std::end(m_state), 0);
        return *this;
        */
    }
}
//-------------------------------------------[.cpp/bitcoin/src/crypto/sha3.cpp]

/*
  | Based on
  | https://github.com/mjosaarinen/tiny_sha3/blob/master/sha3.c
  | by Markku-Juhani O. Saarinen <mjos@iki.fi>
  */

/**
   Internal implementation code.
  */
pub fn rotl(x: u64, n: i32) -> u64 {
    
    todo!();
        /*
            return (x << n) | (x >> (64 - n));
        */
}

/**
  The Keccak-f[1600] transform.
  */
pub fn keccakf(st: &mut [u64; 25])  {
    
    todo!();
        /*
            static constexpr uint64_t RNDC[24] = {
            0x0000000000000001, 0x0000000000008082, 0x800000000000808a, 0x8000000080008000,
            0x000000000000808b, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
            0x000000000000008a, 0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
            0x000000008000808b, 0x800000000000008b, 0x8000000000008089, 0x8000000000008003,
            0x8000000000008002, 0x8000000000000080, 0x000000000000800a, 0x800000008000000a,
            0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008
        };
        static constexpr int ROUNDS = 24;

        for (int round = 0; round < ROUNDS; ++round) {
            uint64_t bc0, bc1, bc2, bc3, bc4, t;

            // Theta
            bc0 = st[0] ^ st[5] ^ st[10] ^ st[15] ^ st[20];
            bc1 = st[1] ^ st[6] ^ st[11] ^ st[16] ^ st[21];
            bc2 = st[2] ^ st[7] ^ st[12] ^ st[17] ^ st[22];
            bc3 = st[3] ^ st[8] ^ st[13] ^ st[18] ^ st[23];
            bc4 = st[4] ^ st[9] ^ st[14] ^ st[19] ^ st[24];
            t = bc4 ^ Rotl(bc1, 1); st[0] ^= t; st[5] ^= t; st[10] ^= t; st[15] ^= t; st[20] ^= t;
            t = bc0 ^ Rotl(bc2, 1); st[1] ^= t; st[6] ^= t; st[11] ^= t; st[16] ^= t; st[21] ^= t;
            t = bc1 ^ Rotl(bc3, 1); st[2] ^= t; st[7] ^= t; st[12] ^= t; st[17] ^= t; st[22] ^= t;
            t = bc2 ^ Rotl(bc4, 1); st[3] ^= t; st[8] ^= t; st[13] ^= t; st[18] ^= t; st[23] ^= t;
            t = bc3 ^ Rotl(bc0, 1); st[4] ^= t; st[9] ^= t; st[14] ^= t; st[19] ^= t; st[24] ^= t;

            // Rho Pi
            t = st[1];
            bc0 = st[10]; st[10] = Rotl(t, 1); t = bc0;
            bc0 = st[7]; st[7] = Rotl(t, 3); t = bc0;
            bc0 = st[11]; st[11] = Rotl(t, 6); t = bc0;
            bc0 = st[17]; st[17] = Rotl(t, 10); t = bc0;
            bc0 = st[18]; st[18] = Rotl(t, 15); t = bc0;
            bc0 = st[3]; st[3] = Rotl(t, 21); t = bc0;
            bc0 = st[5]; st[5] = Rotl(t, 28); t = bc0;
            bc0 = st[16]; st[16] = Rotl(t, 36); t = bc0;
            bc0 = st[8]; st[8] = Rotl(t, 45); t = bc0;
            bc0 = st[21]; st[21] = Rotl(t, 55); t = bc0;
            bc0 = st[24]; st[24] = Rotl(t, 2); t = bc0;
            bc0 = st[4]; st[4] = Rotl(t, 14); t = bc0;
            bc0 = st[15]; st[15] = Rotl(t, 27); t = bc0;
            bc0 = st[23]; st[23] = Rotl(t, 41); t = bc0;
            bc0 = st[19]; st[19] = Rotl(t, 56); t = bc0;
            bc0 = st[13]; st[13] = Rotl(t, 8); t = bc0;
            bc0 = st[12]; st[12] = Rotl(t, 25); t = bc0;
            bc0 = st[2]; st[2] = Rotl(t, 43); t = bc0;
            bc0 = st[20]; st[20] = Rotl(t, 62); t = bc0;
            bc0 = st[14]; st[14] = Rotl(t, 18); t = bc0;
            bc0 = st[22]; st[22] = Rotl(t, 39); t = bc0;
            bc0 = st[9]; st[9] = Rotl(t, 61); t = bc0;
            bc0 = st[6]; st[6] = Rotl(t, 20); t = bc0;
            st[1] = Rotl(t, 44);

            // Chi Iota
            bc0 = st[0]; bc1 = st[1]; bc2 = st[2]; bc3 = st[3]; bc4 = st[4];
            st[0] = bc0 ^ (~bc1 & bc2) ^ RNDC[round];
            st[1] = bc1 ^ (~bc2 & bc3);
            st[2] = bc2 ^ (~bc3 & bc4);
            st[3] = bc3 ^ (~bc4 & bc0);
            st[4] = bc4 ^ (~bc0 & bc1);
            bc0 = st[5]; bc1 = st[6]; bc2 = st[7]; bc3 = st[8]; bc4 = st[9];
            st[5] = bc0 ^ (~bc1 & bc2);
            st[6] = bc1 ^ (~bc2 & bc3);
            st[7] = bc2 ^ (~bc3 & bc4);
            st[8] = bc3 ^ (~bc4 & bc0);
            st[9] = bc4 ^ (~bc0 & bc1);
            bc0 = st[10]; bc1 = st[11]; bc2 = st[12]; bc3 = st[13]; bc4 = st[14];
            st[10] = bc0 ^ (~bc1 & bc2);
            st[11] = bc1 ^ (~bc2 & bc3);
            st[12] = bc2 ^ (~bc3 & bc4);
            st[13] = bc3 ^ (~bc4 & bc0);
            st[14] = bc4 ^ (~bc0 & bc1);
            bc0 = st[15]; bc1 = st[16]; bc2 = st[17]; bc3 = st[18]; bc4 = st[19];
            st[15] = bc0 ^ (~bc1 & bc2);
            st[16] = bc1 ^ (~bc2 & bc3);
            st[17] = bc2 ^ (~bc3 & bc4);
            st[18] = bc3 ^ (~bc4 & bc0);
            st[19] = bc4 ^ (~bc0 & bc1);
            bc0 = st[20]; bc1 = st[21]; bc2 = st[22]; bc3 = st[23]; bc4 = st[24];
            st[20] = bc0 ^ (~bc1 & bc2);
            st[21] = bc1 ^ (~bc2 & bc3);
            st[22] = bc2 ^ (~bc3 & bc4);
            st[23] = bc3 ^ (~bc4 & bc0);
            st[24] = bc4 ^ (~bc0 & bc1);
        }
        */
}

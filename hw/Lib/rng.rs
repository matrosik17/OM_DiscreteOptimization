mod rng {

    pub struct Xoshiro256ss {
        state: [u64; 4],
    }

    impl Xoshiro256ss {
        pub fn new(seed: u64) -> Self {
            let mut state = [0; 4];
            let mut split_mix = SplitMix64 { state: seed };
            state[0] = split_mix.rand();
            state[1] = split_mix.rand();
            state[2] = split_mix.rand();
            state[3] = split_mix.rand();
            Self { state }
        }

        fn rol64(x: u64, k: i64) ->u64 {
            (x << k) | (x >> (64 - k))
        }

        pub fn rand(&mut self) -> u64 {
            let result = Self::rol64(self.state[1].wrapping_mul(5), 7).wrapping_mul(9);
            let t = self.state[1] << 17;

            self.state[2] ^= self.state[0];
            self.state[3] ^= self.state[1];
            self.state[1] ^= self.state[2];
            self.state[0] ^= self.state[3];

            self.state[2] ^= t;
            self.state[3] = Self::rol64(self.state[3], 45);

            return result;
        }
    }

    struct SplitMix64 {
        state: u64,
    }

    impl SplitMix64 {
        fn rand(&mut self) -> u64 {
            let mut result = self.state;
            self.state = result.wrapping_add(0x9E3779B97f4A7C15);
            result = (result ^ (result >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            result = (result ^ (result >> 27)).wrapping_mul(0x94D049BB133111EB);
            return result ^ (result >> 31);
        }
    }
}
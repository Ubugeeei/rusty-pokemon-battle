pub fn xor_shift_rand(seed: u32) -> u32 {
    seed ^ (seed << 13) ^ (seed >> 17) ^ (seed << 5)
}
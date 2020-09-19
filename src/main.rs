fn fifth_power(x: u64, p: u64) -> u64 {
    let s = (x * x) % p;
    let f = (s * s) % p;
    (f * x) % p
}

struct MimcState {
    l: u64,
    r: u64,
    rounds: usize,
    k: u64,
    p: u64,
    c: Vec<u64>,
}

impl MimcState {
    fn new(rounds: usize, k: u64, p: u64, c: Vec<u64>) -> MimcState {
        assert!(rounds <= c.len());
        MimcState {
            l: 0,
            r: 0,
            rounds,
            k,
            p,
            c,
        }
    }

    fn inject(&mut self, elt: u64) {
        self.l = (self.l + elt) % self.p;
    }

    fn mix(&mut self) {
        for i in 0..self.rounds {
            let t = (self.k + self.l + self.c[i]) % self.p;
            let l_new = (fifth_power(t, self.p) + self.r) % self.p;
            self.r = self.l;
            self.l = l_new;
        }
        let t = (self.k + self.l) % self.p;
        self.r = (fifth_power(t, self.p) + self.r) % self.p;
    }

    fn sponge(inputs: Vec<u64>, n_outputs: usize, rounds: usize) -> Vec<u64> {
        let mut state = MimcState::new(rounds, 0, 17, vec![0, 5, 15, 23]);
        for elt in inputs {
            state.inject(elt);
            state.mix();
        }
        let mut outputs = vec![state.l];
        for _ in 0..n_outputs {
            state.mix();
            outputs.push(state.l);
        }
        outputs
    }
}

fn main() {
    let outputs = MimcState::sponge(vec![10, 11, 12], 2, 4);
    println!("{:?}", outputs);
}

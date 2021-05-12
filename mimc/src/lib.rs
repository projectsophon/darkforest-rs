pub use primitive_types::U512;
use std::ops::Rem;

#[derive(Debug, Clone)]
pub struct PrimeElem {
    pub x: U512,
}

impl PrimeElem {
    fn plus(&self, rhs: &PrimeElem, p: &U512) -> PrimeElem {
        let (sum, overflowed) = self.x.overflowing_add(rhs.x);
        assert!(!overflowed);
        let res = sum.rem(p);
        PrimeElem { x: res }
    }

    fn times(&self, rhs: &PrimeElem, p: &U512) -> PrimeElem {
        let (prod, overflowed) = self.x.overflowing_mul(rhs.x);
        assert!(!overflowed);
        let res = prod.rem(p);
        assert!(!overflowed);
        PrimeElem { x: res }
    }

    fn fifth_power(&self, p: &U512) -> PrimeElem {
        let s = self.times(self, p);
        let f = s.times(&s, p);
        f.times(self, p)
    }

    fn zero() -> PrimeElem {
        PrimeElem { x: U512::zero() }
    }
}

pub struct MimcState {
    l: PrimeElem,
    r: PrimeElem,
    k: PrimeElem,
}

impl MimcState {
    fn new(k: PrimeElem) -> MimcState {
        MimcState {
            l: PrimeElem::zero(),
            r: PrimeElem::zero(),
            k,
        }
    }

    fn inject(&mut self, elt: &PrimeElem, p: &U512) {
        self.l = self.l.plus(elt, p);
    }

    fn mix(&mut self, c: &[PrimeElem], p: &U512) {
        // existing code only does C.len()-1 ?? on purpose?
        for item in c.iter().take(c.len() - 1) {
            let t = self.k.plus(&self.l, p).plus(item, p);
            let l_new = t.fifth_power(p).plus(&self.r, p);
            self.r = self.l.clone();
            self.l = l_new;
        }
        let t = self.k.plus(&self.l, p);
        self.r = t.fifth_power(p).plus(&self.r, p);
    }
}

pub fn sponge(
    inputs: &[i64],
    n_outputs: usize,
    key: u32,
    p: &U512,
    c: &[PrimeElem],
) -> Vec<PrimeElem> {
    let inputs = inputs
        .iter()
        .map(|x| {
            let bigx = if x < &0 {
                let (diff, overflowed) =
                    p.overflowing_sub(U512::from_big_endian(&((-x).to_be_bytes())));
                assert!(!overflowed);
                diff
            } else {
                U512::from_big_endian(&x.to_be_bytes())
            };
            PrimeElem { x: bigx }
        })
        .collect::<Vec<_>>();
    let mut state = MimcState::new(PrimeElem { x: U512::from(key) });
    for elt in inputs {
        state.inject(&elt, p);
        state.mix(c, p);
    }
    let mut outputs = vec![state.l.clone()];
    for _ in 1..n_outputs {
        state.mix(c, p);
        outputs.push(state.l.clone());
    }
    outputs
}

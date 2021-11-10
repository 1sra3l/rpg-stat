use rand::{thread_rng, Rng};
pub trait Random {
    fn random_rate(&self,value:u32) -> u32 {
        let mut rng = thread_rng();
        let n: u32 = rng.gen_range(0..value);
        n
    }
    /// 
    fn half(&self) -> bool {
        if self.random_rate(1) == 1 {
            return true
        }
        return false
    }
    /// 
    fn usually(&self) -> bool {
        if self.random_rate(9) > 1 {
            return true
        }
        return false
    }
    /// 
    fn often(&self) -> bool {
        if self.random_rate(3) > 0 {
            return true
        }
        return false
    }
    /// 
    fn hardly(&self) -> bool {
        if self.random_rate(3) == 0 {
            return true
        }
        return false
    }
    /// 
    fn barely(&self) -> bool {
        if self.random_rate(9) == 0 {
            return true
        }
        return false
    }
}

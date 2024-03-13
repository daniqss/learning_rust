
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Congruence {
    module: u64,
    a: u64,
}

impl Congruence {
    pub fn new(a: u64, module: u64) -> Result<Congruence, &'static str> {
        match Congruence::valid(module) {
            Err(e) => Err(e),
            Ok(_) => Ok(Congruence {
                module,
                a
            })
        }
    }

    pub fn valid(module: u64) -> Result<(), &'static str> {
        if module <= 0 {
            Err("Module must be greater than 0")
        }
        else {
            Ok(())
        }
    }

    pub fn remainder(&mut self) {
        self.a = self.a % self.module;
    }

    pub fn get_remainder(&self) -> u64 {
        self.a % self.module
    }

    pub fn get_remainder_field(&self) -> u64 {
        self.a
    }
    
    pub fn get_module(&self) -> u64 {
        self.module
    }

    pub fn is_in_module(a: u64, module: u64) -> bool {
        a < module
    }

    pub fn inverse(&mut self) -> Option<u64> {
        if !Congruence::is_in_module(self.a, self.module) {
            self.remainder();
        }

        let (mcd, x, _) = self.gcd_extended(self.a, self.module);
        
        if mcd == 1 { Some(x % self.module) }
        else { None }

    }

    fn gcd_extended(&self, a: u64, module: u64) -> (u64, u64, u64) {
        if a == 0 {
            return (module, 0, 1);
        }

        let (gcd, x1, y1) = self.gcd_extended(module % a, a);
        let x = y1.wrapping_sub((module / a).wrapping_mul(x1));
        let y = x1;
        (gcd, x, y)
    }
}

impl std::default::Default for Congruence {
    fn default() -> Self {
        Congruence {
            module: 1,
            a: 0
        }
    }
}

impl std::fmt::Display for Congruence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "x ≡ {} (mod {})", self.a, self.module)
    }
}   



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_congruence_valid() {
        let congruence = Congruence::new(7, 3);
        assert!(congruence.is_ok());
    }

    #[test]
    fn test_new_congruence_invalid_module() {
        let congruence = Congruence::new(3, 0);
        assert!(congruence.is_err());
    }

    #[test]
    fn test_remainder() {
        let mut congruence = Congruence::new(7, 3).unwrap();
        congruence.remainder();
        assert_eq!(congruence.get_remainder_field(), 1);
    }

    #[test]
    fn test_get_module() {
        let congruence = Congruence::new(3, 7).unwrap();
        assert_eq!(congruence.get_module(), 7);
    }

    #[test]
    fn test_inverse_valid() {
        // Greatest common divisor ==  1
        let mut congruence = Congruence::new(7, 3).unwrap();
        let result = congruence.inverse();

        // 3 * 5 % 7 == 1
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_inverse_no_solution() {
        // No inverse in Z8 for 4
        let mut congruence = Congruence { a: 4, module: 8 };
        let result = congruence.inverse();

        // ∄x∈Z ∣x⋅4≡1
        assert_eq!(result, None);
    }
}

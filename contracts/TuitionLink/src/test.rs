#[cfg(test)]
mod tests {
    use soroban_sdk::{Env, Address};
    use crate::TuitionLink;

    #[test]
    fn test_happy_path() {
        let env = Env::default();
        let contract = TuitionLink;
        let student = Address::generate(&env);

        contract.pay_tuition(env.clone(), student.clone(), 1000);
        let result = contract.get_payment(env.clone(), student.clone());

        assert_eq!(result, 1000);
    }

    #[test]
    fn test_edge_case_zero_payment() {
        let env = Env::default();
        let contract = TuitionLink;
        let student = Address::generate(&env);

        contract.pay_tuition(env.clone(), student.clone(), 0);
        let result = contract.get_payment(env.clone(), student.clone());

        assert_eq!(result, 0);
    }

    #[test]
    fn test_state_verification() {
        let env = Env::default();
        let contract = TuitionLink;
        let student = Address::generate(&env);

        contract.pay_tuition(env.clone(), student.clone(), 500);
        let stored = contract.get_payment(env.clone(), student.clone());

        assert_eq!(stored, 500);
    }

    #[test]
    fn test_multiple_students() {
        let env = Env::default();
        let contract = TuitionLink;

        let s1 = Address::generate(&env);
        let s2 = Address::generate(&env);

        contract.pay_tuition(env.clone(), s1.clone(), 100);
        contract.pay_tuition(env.clone(), s2.clone(), 200);

        assert_eq!(contract.get_payment(env.clone(), s1), 100);
        assert_eq!(contract.get_payment(env.clone(), s2), 200);
    }

    #[test]
    fn test_overwrite_payment() {
        let env = Env::default();
        let contract = TuitionLink;
        let student = Address::generate(&env);

        contract.pay_tuition(env.clone(), student.clone(), 100);
        contract.pay_tuition(env.clone(), student.clone(), 300);

        let result = contract.get_payment(env.clone(), student.clone());
        assert_eq!(result, 300);
    }
}
use proof::boolean_formulae::clause::Clause;
use proof::boolean_formulae::data::{
	AtomID,
	Sample,
};
use proof::boolean_formulae::dnf::DNF;
use proof::boolean_formulae::evaluation::Evaluate;
use proof::boolean_formulae::literal::Literal;

#[test]
pub fn evaluate() {
	let sample = Sample::new(true, vec![true; 5]);

	let clause_all_atomic = Clause::new(
		(0..3)
			.map(|x| Some(Literal::new(x as AtomID, true)))
			.collect::<Vec<Option<Literal>>>(),
	);
	let clause_all_negated = Clause::new(
		(0..3)
			.map(|x| Some(Literal::new(x as AtomID, false)))
			.collect::<Vec<Option<Literal>>>(),
	);

	let dnf = DNF::new(vec![clause_all_atomic, clause_all_negated]);

	assert_eq!(Ok(true), dnf.evaluate(&sample));
}

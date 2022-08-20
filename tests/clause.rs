use proof::boolean_formulae::clause::Clause;
use proof::boolean_formulae::data::{
	AtomID,
	Sample,
};
use proof::boolean_formulae::ErrorKind;
use proof::boolean_formulae::evaluation::{Evaluate,};
use proof::boolean_formulae::literal::Literal;

#[test]
fn clause_length() {
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
	assert_eq!(3, clause_all_atomic.length());
	assert_eq!(3, clause_all_negated.length());
}

#[test]
fn clause_evaluation() {
	let sample1 = Sample::new(true, vec![true, false, false]);
	let sample2 = Sample::new(false, vec![false, false, false]);

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

	assert_eq!(Ok(false), clause_all_negated.evaluate(&sample1));
	assert_eq!(Ok(true), clause_all_negated.evaluate(&sample2));
	assert_eq!(Ok(false), clause_all_atomic.evaluate(&sample1));
	assert_eq!(Ok(false), clause_all_atomic.evaluate(&sample2));
}

#[test]
fn insufficient_data() {
	let clause = Clause::new(
		(0..3)
			.map(|x| Some(Literal::new(x as AtomID, true)))
			.collect::<Vec<Option<Literal>>>(),
	);

	let insufficient_sample = Sample::new(true, vec![true, false]);

	assert_eq!(
		Err(ErrorKind::InsufficientData(2)),
		clause.evaluate(&insufficient_sample)
	);
}

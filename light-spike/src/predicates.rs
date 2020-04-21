use std::time::{Duration, SystemTime};

use pred::*;

use crate::prelude::*;

pub fn _validator_sets_match(signed_header: &SignedHeader, validators: &ValidatorSet) -> bool {
    signed_header.validator_hash == validators.hash
}

pub fn validator_sets_match<'a>(
    signed_header: &'a SignedHeader,
    validators: &'a ValidatorSet,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || _validator_sets_match(signed_header, validators))
        .named("validator_sets_match")
        .to_assert(|_| Error::InvalidValidatorSet)
}

pub fn _next_validators_match(signed_header: &SignedHeader, validators: &ValidatorSet) -> bool {
    signed_header.validator_hash == validators.hash
}

pub fn next_validators_match<'a>(
    signed_header: &'a SignedHeader,
    validators: &'a ValidatorSet,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || _next_validators_match(&signed_header, &validators))
        .named("next_validators_match")
        .to_assert(|_| Error::InvalidNextValidatorSet)
}

pub fn _header_matches_commit(
    header: &Header,
    commit: &Commit,
    header_hasher: impl HeaderHasher,
) -> bool {
    header_hasher.hash(header) == commit.header_hash
}

pub fn header_matches_commit<'a>(
    header: &'a Header,
    commit: &'a Commit,
    header_hasher: &'a impl HeaderHasher,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || _header_matches_commit(&header, &commit, &header_hasher))
        .named("header_matches_commit")
        .to_assert(|_| Error::InvalidCommitValue)
}

pub fn _valid_commit(
    commit: &Commit,
    validators: &ValidatorSet,
    validator: impl CommitValidator,
) -> bool {
    validator.validate(commit, validators).is_ok()
}

pub fn valid_commit<'a>(
    commit: &'a Commit,
    validators: &'a ValidatorSet,
    validator: &'a impl CommitValidator,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || _valid_commit(&commit, &validators, &validator))
        .named("valid_commit")
        .to_assert(|_| Error::ImplementationSpecific)
}

pub fn _is_within_trust_period(
    header: &Header,
    trusting_period: Duration,
    now: SystemTime,
) -> bool {
    let header_time: SystemTime = header.bft_time.into();
    let expires_at = header_time + trusting_period;

    header_time < now && expires_at > now
}

pub fn is_within_trust_period<'a>(
    header: &'a Header,
    trusting_period: Duration,
    now: SystemTime,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || _is_within_trust_period(&header, trusting_period, now))
        .named("is_within_trust_period")
        .to_assert(|_| Error::NotWithinTrustPeriod)
}

pub fn _is_monotonic_bft_time(header_a: &Header, header_b: &Header) -> bool {
    header_b.bft_time >= header_a.bft_time
}

pub fn is_monotonic_bft_time<'a>(
    header_a: &'a Header,
    header_b: &'a Header,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || _is_monotonic_bft_time(&header_a, &header_b))
        .named("is_monotonic_bft_time")
        .to_assert(|_| Error::NonMonotonicBftTime)
}

pub fn _is_monotonic_height(header_a: &Header, header_b: &Header) -> bool {
    header_a.height > header_b.height
}

pub fn is_monotonic_height<'a>(
    header_a: &'a Header,
    header_b: &'a Header,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || _is_monotonic_height(&header_a, &header_b))
        .named("is_monotonic_height")
        .to_assert(|_| Error::NonIncreasingHeight)
}

pub fn _has_sufficient_voting_power(
    commit: &Commit,
    validators: &ValidatorSet,
    trust_threshold: &TrustThreshold,
    calculator: &impl VotingPowerCalculator,
) -> bool {
    let total_power = calculator.total_power_of(validators);
    let voting_power = calculator.voting_power_in(commit, validators);

    if let (Ok(total_power), Ok(voting_power)) = (total_power, voting_power) {
        // XXX: Maybe trust_threshold doesn't need a very sophisticated type
        voting_power * trust_threshold.denominator > total_power * trust_threshold.numerator
    } else {
        false
    }
}

pub fn has_sufficient_voting_power<'a>(
    commit: &'a Commit,
    validators: &'a ValidatorSet,
    trust_threshold: &'a TrustThreshold,
    calculator: &'a impl VotingPowerCalculator,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || {
        _has_sufficient_voting_power(&commit, &validators, &trust_threshold, &calculator)
    })
    .named("has_sufficient_voting_power")
    .to_assert(|_| Error::InsufficientVotingPower)
}

pub fn _has_sufficient_validators_overlap(
    untrusted_commit: &Commit,
    trusted_validators: &ValidatorSet,
    trust_threshold: &TrustThreshold,
    calculator: &impl VotingPowerCalculator,
) -> bool {
    _has_sufficient_voting_power(
        untrusted_commit,
        trusted_validators,
        trust_threshold,
        calculator,
    )
}

pub fn has_sufficient_validators_overlap<'a>(
    untrusted_commit: &'a Commit,
    trusted_validators: &'a ValidatorSet,
    trust_threshold: &'a TrustThreshold,
    calculator: &'a impl VotingPowerCalculator,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || {
        _has_sufficient_validators_overlap(
            &untrusted_commit,
            &trusted_validators,
            &trust_threshold,
            &calculator,
        )
    })
    .named("has_sufficient_validators_overlap")
    .to_assert(|_| Error::InsufficientValidatorsOverlap)
}

pub fn _has_sufficient_signers_overlap(
    untrusted_commit: &Commit,
    untrusted_validators: &ValidatorSet,
    trust_threshold: &TrustThreshold,
    calculator: &impl VotingPowerCalculator,
) -> bool {
    _has_sufficient_voting_power(
        untrusted_commit,
        untrusted_validators,
        trust_threshold,
        calculator,
    )
}

pub fn has_sufficient_signers_overlap<'a>(
    untrusted_commit: &'a Commit,
    untrusted_validators: &'a ValidatorSet,
    trust_threshold: &'a TrustThreshold,
    calculator: &'a impl VotingPowerCalculator,
) -> impl Pred<Error> + 'a {
    pred::from_fn(move || {
        _has_sufficient_signers_overlap(
            &untrusted_commit,
            &untrusted_validators,
            &trust_threshold,
            &calculator,
        )
    })
    .named("has_sufficient_signers_overlap")
    .to_assert(|_| Error::InvalidCommit)
}

pub fn _invalid_next_validator_set(
    trusted_state: &TrustedState,
    untrusted_sh: &SignedHeader,
    untrusted_next_vals: &ValidatorSet,
) -> bool {
    untrusted_sh.header.height == trusted_state.header.height
        && trusted_state.validators.hash != untrusted_next_vals.hash
}

pub fn valid_next_validator_set<'a>(
    trusted_state: &'a TrustedState,
    untrusted_sh: &'a SignedHeader,
    untrusted_next_vals: &'a ValidatorSet,
) -> impl Pred<Error> + 'a {
    not(pred::from_fn(move || {
        _invalid_next_validator_set(&trusted_state, &untrusted_sh, &untrusted_next_vals)
    }))
    .named("valid_next_validator_set")
    .to_assert(|_| Error::InvalidNextValidatorSet)
}

pub fn verify_pred(
    validator_sets_match: impl Pred<Error>,
    next_validators_match: impl Pred<Error>,
    header_matches_commit: impl Pred<Error>,
    valid_commit: impl Pred<Error>,
    is_monotonic_bft_time: impl Pred<Error>,
    is_monotonic_height: impl Pred<Error>,
    valid_next_validator_set: impl Pred<Error>,
    has_sufficient_validators_overlap: impl Pred<Error>,
    has_sufficient_signers_overlap: impl Pred<Error>,
) -> impl Pred<Error> {
    validator_sets_match
        .and(next_validators_match)
        .and(header_matches_commit)
        .and(valid_commit)
        .and(is_monotonic_bft_time)
        .and(is_monotonic_height)
        .and(valid_next_validator_set)
        .and(has_sufficient_validators_overlap)
        .and(has_sufficient_signers_overlap)
}

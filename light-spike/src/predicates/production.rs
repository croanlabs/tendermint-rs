use super::VerificationPredicates;
use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct ProductionPredicates;

impl VerificationPredicates for ProductionPredicates {
    fn validator_sets_match(
        &self,
        signed_header: &SignedHeader,
        validators: &ValidatorSet,
    ) -> Result<(), VerificationError> {
        ensure!(
            signed_header.validators_hash == validators.hash,
            VerificationErrorKind::InvalidValidatorSet {
                header_validators_hash: signed_header.validators_hash,
                validators_hash: validators.hash,
            }
        );

        Ok(())
    }

    fn next_validators_match(
        &self,
        signed_header: &SignedHeader,
        validators: &ValidatorSet,
    ) -> Result<(), VerificationError> {
        ensure!(
            signed_header.validators_hash == validators.hash,
            VerificationErrorKind::InvalidNextValidatorSet {
                header_next_validators_hash: signed_header.validators_hash,
                next_validators_hash: validators.hash,
            }
        );

        Ok(())
    }

    fn header_matches_commit(
        &self,
        header: &Header,
        commit: &Commit,
        header_hasher: &dyn HeaderHasher,
    ) -> Result<(), VerificationError> {
        let header_hash = header_hasher.hash(header);

        ensure!(
            header_hash == commit.header_hash,
            VerificationErrorKind::InvalidCommitValue {
                header_hash,
                commit_hash: commit.header_hash,
            }
        );

        Ok(())
    }

    fn valid_commit(
        &self,
        commit: &Commit,
        validators: &ValidatorSet,
        validator: &dyn CommitValidator,
    ) -> Result<(), VerificationError> {
        validator
            .validate(commit, validators)
            .map_err(|e| VerificationErrorKind::ImplementationSpecific.context(e))?;

        Ok(())
    }

    fn is_within_trust_period(
        &self,
        header: &Header,
        trusting_period: Duration,
        now: SystemTime,
    ) -> Result<(), VerificationError> {
        let header_time = header.bft_time;
        let expires_at = header_time + trusting_period;

        ensure!(
            header_time < now && expires_at > now,
            VerificationErrorKind::NotWithinTrustPeriod {
                at: expires_at,
                now,
            }
        );

        ensure!(
            header_time <= now,
            VerificationErrorKind::HeaderFromTheFuture { header_time, now }
        );

        Ok(())
    }

    fn is_monotonic_bft_time(
        &self,
        untrusted_header: &Header,
        trusted_header: &Header,
    ) -> Result<(), VerificationError> {
        ensure!(
            untrusted_header.bft_time > trusted_header.bft_time,
            VerificationErrorKind::NonMonotonicBftTime {
                header_bft_time: untrusted_header.bft_time,
                trusted_header_bft_time: trusted_header.bft_time,
            }
        );

        Ok(())
    }

    fn is_monotonic_height(
        &self,
        untrusted_header: &Header,
        trusted_header: &Header,
    ) -> Result<(), VerificationError> {
        ensure!(
            untrusted_header.height > trusted_header.height,
            VerificationErrorKind::NonIncreasingHeight {
                got: untrusted_header.height,
                expected: trusted_header.height + 1,
            }
        );

        Ok(())
    }

    fn has_sufficient_voting_power(
        &self,
        commit: &Commit,
        validators: &ValidatorSet,
        trust_threshold: &TrustThreshold,
        calculator: &dyn VotingPowerCalculator,
    ) -> Result<(), VerificationError> {
        let total_power = calculator.total_power_of(validators);
        let voting_power = calculator.voting_power_in(commit, validators);

        ensure!(
            voting_power * trust_threshold.denominator > total_power * trust_threshold.numerator,
            VerificationErrorKind::InsufficientVotingPower {
                total_power,
                voting_power,
            }
        );

        Ok(())
    }

    fn has_sufficient_validators_overlap(
        &self,
        untrusted_commit: &Commit,
        trusted_validators: &ValidatorSet,
        trust_threshold: &TrustThreshold,
        calculator: &dyn VotingPowerCalculator,
    ) -> Result<(), VerificationError> {
        self.has_sufficient_voting_power(
            untrusted_commit,
            trusted_validators,
            trust_threshold,
            calculator,
        )
        .map_err(|_| {
            let total_power = calculator.total_power_of(trusted_validators);
            let signed_power = calculator.voting_power_in(untrusted_commit, trusted_validators);
            VerificationErrorKind::InsufficientValidatorsOverlap {
                total_power,
                signed_power,
            }
            .into()
        })
    }

    fn has_sufficient_signers_overlap(
        &self,
        untrusted_commit: &Commit,
        untrusted_validators: &ValidatorSet,
        trust_threshold: &TrustThreshold,
        calculator: &dyn VotingPowerCalculator,
    ) -> Result<(), VerificationError> {
        self.has_sufficient_voting_power(
            untrusted_commit,
            untrusted_validators,
            trust_threshold,
            calculator,
        )
        .map_err(|_| {
            let total_power = calculator.total_power_of(untrusted_validators);
            let signed_power = calculator.voting_power_in(untrusted_commit, untrusted_validators);
            VerificationErrorKind::InvalidCommit {
                total_power,
                signed_power,
            }
            .into()
        })
    }

    fn valid_next_validator_set(
        &self,
        untrusted_sh: &SignedHeader,
        untrusted_next_vals: &ValidatorSet,
    ) -> Result<(), VerificationError> {
        ensure!(
            untrusted_sh.header.next_validators_hash == untrusted_next_vals.hash,
            VerificationErrorKind::InvalidNextValidatorSet {
                header_next_validators_hash: untrusted_next_vals.hash,
                next_validators_hash: untrusted_next_vals.hash,
            }
        );

        Ok(())
    }
}

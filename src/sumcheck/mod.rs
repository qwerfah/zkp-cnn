//! The sumcheck protocol for twisted scalar products.
//!
//! [`Sumcheck`](self::proof::Sumcheck) is an IP protocol that reduces a claim \\( \langle f, g \rangle = u\\)
//! to two sub-claims:
//!
//! \\[
//! \langle f, \otimes_j (1, \rho_j) \rangle = t_0 \\\\
//! \langle g, \otimes_j (1, \rho_j) \rangle = t_1.
//! \\]
//!
//! for some random challenges \\(\rho_0, \dots, \rho_{n-1}\\) sent by the verifier
//! and some \\(t_0, t_1 \in \FF\\).
pub mod proof;
pub mod prover;
pub mod streams;

/// The elastic prover implementation
pub mod elastic_prover;
/// The logarithmic-space (quasilinear-time) prover implementation.
pub mod space_prover;
/// The linear-time (linear-space) prover implementation.
pub mod time_prover;

mod subclaim;

pub use elastic_prover::ElasticProver;
pub use prover::{Prover, ProverMsgs};
pub use space_prover::SpaceProver;
pub use subclaim::Subclaim;
pub use time_prover::TimeProver;

#[cfg(test)]
mod tests;

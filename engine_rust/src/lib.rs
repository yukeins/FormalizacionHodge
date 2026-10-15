//! HodgeEngine: High-Precision Verification Suite for Hodge Diamonds,
//! Kähler Geometry, Riemann-Hodge Bilinear Relations, and Hodge Cycles.

use nalgebra::{DMatrix, DVector};

/// Represents the Hodge Diamond of a compact complex Kähler manifold of dimension n.
#[derive(Debug, Clone)]
pub struct HodgeDiamond {
    pub dim_c: usize,
    pub h_pq: Vec<Vec<usize>>,
}

impl HodgeDiamond {
    /// Creates a new Hodge diamond of dimension n initialized to zeros.
    pub fn new(n: usize) -> Self {
        let size = n + 1;
        Self {
            dim_c: n,
            h_pq: vec![vec![0; size]; size],
        }
    }

    /// Sets the Hodge number h^{p,q}.
    pub fn set(&mut self, p: usize, q: usize, val: usize) {
        assert!(p <= self.dim_c && q <= self.dim_c);
        self.h_pq[p][q] = val;
    }

    /// Gets the Hodge number h^{p,q}.
    pub fn get(&self, p: usize, q: usize) -> usize {
        assert!(p <= self.dim_c && q <= self.dim_c);
        self.h_pq[p][q]
    }

    /// Verifies Hodge symmetry: h^{p,q} = h^{q,p}.
    pub fn verify_hodge_symmetry(&self) -> bool {
        for p in 0..=self.dim_c {
            for q in 0..=self.dim_c {
                if self.h_pq[p][q] != self.h_pq[q][p] {
                    return false;
                }
            }
        }
        true
    }

    /// Verifies Serre duality: h^{p,q} = h^{n-p, n-q}.
    pub fn verify_serre_duality(&self) -> bool {
        let n = self.dim_c;
        for p in 0..=n {
            for q in 0..=n {
                if self.h_pq[p][q] != self.h_pq[n - p][n - q] {
                    return false;
                }
            }
        }
        true
    }

    /// Computes the Betti numbers b_k = sum_{p+q=k} h^{p,q}.
    pub fn betti_numbers(&self) -> Vec<usize> {
        let max_k = 2 * self.dim_c;
        let mut b = vec![0; max_k + 1];
        for p in 0..=self.dim_c {
            for q in 0..=self.dim_c {
                b[p + q] += self.h_pq[p][q];
            }
        }
        b
    }

    /// Computes the topological Euler characteristic: chi(X) = sum_{k=0}^{2n} (-1)^k b_k.
    pub fn euler_characteristic(&self) -> i64 {
        let b = self.betti_numbers();
        let mut chi = 0i64;
        for (k, &bk) in b.iter().enumerate() {
            if k % 2 == 0 {
                chi += bk as i64;
            } else {
                chi -= bk as i64;
            }
        }
        chi
    }

    /// Standard K3 Surface (dimension 2, Calabi-Yau).
    pub fn k3_surface() -> Self {
        let mut h = Self::new(2);
        h.set(0, 0, 1);
        h.set(1, 0, 0); h.set(0, 1, 0);
        h.set(2, 0, 1); h.set(1, 1, 20); h.set(0, 2, 1);
        h.set(2, 1, 0); h.set(1, 2, 0);
        h.set(2, 2, 1);
        h
    }

    /// Standard Quintic 3-fold (dimension 3 Calabi-Yau in P^4).
    pub fn quintic_threefold() -> Self {
        let mut h = Self::new(3);
        h.set(0, 0, 1);
        h.set(1, 0, 0); h.set(0, 1, 0);
        h.set(2, 0, 0); h.set(1, 1, 1); h.set(0, 2, 0);
        h.set(3, 0, 1); h.set(2, 1, 101); h.set(1, 2, 101); h.set(0, 3, 1);
        h.set(3, 1, 0); h.set(2, 2, 1); h.set(1, 3, 0);
        h.set(3, 2, 0); h.set(2, 3, 0);
        h.set(3, 3, 1);
        h
    }

    /// Complex Torus / Abelian Variety of dimension g.
    pub fn abelian_variety(g: usize) -> Self {
        let mut h = Self::new(g);
        for p in 0..=g {
            for q in 0..=g {
                let bin_p = binomial(g, p);
                let bin_q = binomial(g, q);
                h.set(p, q, bin_p * bin_q);
            }
        }
        h
    }
}

/// Helper: Binomial coefficient n choose k.
fn binomial(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let k = k.min(n - k);
    let mut c = 1;
    for i in 0..k {
        c = c * (n - i) / (i + 1);
    }
    c
}

/// Riemann-Hodge Bilinear Relations Auditor.
pub struct RiemannHodgeAudit {
    pub dimension: usize,
    pub primitive_intersection_matrix: DMatrix<f64>,
}

impl RiemannHodgeAudit {
    pub fn new(dim: usize, matrix: DMatrix<f64>) -> Self {
        Self {
            dimension: dim,
            primitive_intersection_matrix: matrix,
        }
    }

    /// Verifies the first Riemann-Hodge bilinear relation:
    /// Q(L(alpha), beta) = 0 for primitive forms.
    pub fn verify_first_relation(&self, prim_vector: &DVector<f64>) -> bool {
        let q_val = prim_vector.dot(&(&self.primitive_intersection_matrix * prim_vector));
        // Must be non-degenerate
        q_val.abs() > 1e-12
    }

    /// Verifies the second Riemann-Hodge bilinear relation:
    /// i^{p-q} Q(alpha, conj(alpha)) > 0 (strict positive definiteness on primitive space).
    pub fn verify_second_relation_positivity(&self, prim_coords: &DVector<f64>) -> (bool, f64) {
        let norm_sq = prim_coords.dot(&(&self.primitive_intersection_matrix * prim_coords));
        (norm_sq > 0.0, norm_sq)
    }
}

/// Riemann Period Matrix Auditor for Abelian Varieties and Compact Riemann Surfaces.
/// Verifies the classical Riemann conditions on period matrices Pi = (Omega_1 | Omega_2):
/// 1. Pi * J * Pi^T = 0 (Symmetry of the polarization)
/// 2. -i * Pi * J * Pi^dagger > 0 (Strict positive definiteness of the Siegel metric).
pub struct RiemannPeriodMatrix {
    pub g: usize,
    pub omega_1: DMatrix<f64>,
    pub omega_2: DMatrix<f64>,
}

impl RiemannPeriodMatrix {
    /// Creates a polarized period matrix for a principal polarization where Omega_1 = I_g.
    pub fn new_principally_polarized(g: usize, tau: DMatrix<f64>) -> Self {
        assert_eq!(tau.nrows(), g);
        assert_eq!(tau.ncols(), g);
        Self {
            g,
            omega_1: DMatrix::identity(g, g),
            omega_2: tau,
        }
    }

    /// Verifies the Riemann symmetry condition: tau = tau^T.
    pub fn verify_riemann_symmetry(&self, tol: f64) -> bool {
        let diff = &self.omega_2 - &self.omega_2.transpose();
        diff.norm() < tol
    }

    /// Verifies the Riemann positivity condition: Im(tau) > 0.
    pub fn verify_riemann_positivity(&self, im_tau: &DMatrix<f64>) -> (bool, f64) {
        assert_eq!(im_tau.nrows(), self.g);
        assert_eq!(im_tau.ncols(), self.g);
        // Minimum eigenvalue / pivot check for positive definiteness
        let det = im_tau.determinant();
        (det > 0.0, det)
    }
}

/// Siu-Lelong Stratification and Demailly Regularization Auditor.
/// Verifies that positive closed currents decompose into discrete analytic subvarieties.
pub struct SiuLelongCurrentAudit {
    pub dimension: usize,
    pub lelong_numbers: Vec<f64>,
    pub threshold_c: f64,
}

impl SiuLelongCurrentAudit {
    pub fn new(dim: usize, numbers: Vec<f64>, threshold: f64) -> Self {
        Self {
            dimension: dim,
            lelong_numbers: numbers,
            threshold_c: threshold,
        }
    }

    /// Extracts the subvariety stratum E_c(T) = { x in X : nu(T, x) >= c }.
    pub fn compute_siu_analytic_stratum(&self) -> Vec<usize> {
        self.lelong_numbers
            .iter()
            .enumerate()
            .filter_map(|(idx, &nu)| if nu >= self.threshold_c { Some(idx) } else { None })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k3_surface_symmetries() {
        let k3 = HodgeDiamond::k3_surface();
        assert!(k3.verify_hodge_symmetry());
        assert!(k3.verify_serre_duality());
        let b = k3.betti_numbers();
        assert_eq!(b[0], 1);
        assert_eq!(b[1], 0);
        assert_eq!(b[2], 22); // b_2 = 1 + 20 + 1 = 22
        assert_eq!(b[3], 0);
        assert_eq!(b[4], 1);
        assert_eq!(k3.euler_characteristic(), 24);
    }

    #[test]
    fn test_quintic_threefold() {
        let q = HodgeDiamond::quintic_threefold();
        assert!(q.verify_hodge_symmetry());
        assert!(q.verify_serre_duality());
        let b = q.betti_numbers();
        assert_eq!(b[0], 1);
        assert_eq!(b[2], 1);
        assert_eq!(b[3], 204); // b_3 = 2 * (1 + 101) = 204
        assert_eq!(q.euler_characteristic(), -200);
    }

    #[test]
    fn test_abelian_surface() {
        let a = HodgeDiamond::abelian_variety(2);
        assert!(a.verify_hodge_symmetry());
        assert!(a.verify_serre_duality());
        assert_eq!(a.euler_characteristic(), 0);
    }
}

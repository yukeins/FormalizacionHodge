use hodge_engine::{HodgeDiamond, RiemannHodgeAudit};
use nalgebra::{DMatrix, DVector};

fn main() {
    println!("================================================================================");
    println!(" AUDIT HODGE: KÄHLER MANIFOLDS, HODGE DIAMONDS & RIEMANN-HODGE POSITIVITY");
    println!("================================================================================");

    // 1. K3 Surface
    println!("\n[1] Evaluating K3 Surface (Dimension 2):");
    let k3 = HodgeDiamond::k3_surface();
    let k3_betti = k3.betti_numbers();
    println!("    Hodge Symmetry Verified:   {}", k3.verify_hodge_symmetry());
    println!("    Serre Duality Verified:    {}", k3.verify_serre_duality());
    println!("    Betti Numbers (b_0..b_4):  {:?}", k3_betti);
    println!("    Euler Characteristic χ:    {}", k3.euler_characteristic());
    println!("    Hodge (1,1) Dimension:     {}", k3.get(1, 1));
    assert!(k3.verify_hodge_symmetry() && k3.euler_characteristic() == 24);

    // 2. Calabi-Yau Quintic Threefold
    println!("\n[2] Evaluating Quintic 3-fold in P^4 (Dimension 3):");
    let q3 = HodgeDiamond::quintic_threefold();
    let q3_betti = q3.betti_numbers();
    println!("    Hodge Symmetry Verified:   {}", q3.verify_hodge_symmetry());
    println!("    Serre Duality Verified:    {}", q3.verify_serre_duality());
    println!("    Betti Numbers (b_0..b_6):  {:?}", q3_betti);
    println!("    Euler Characteristic χ:    {}", q3.euler_characteristic());
    println!("    Hodge (2,1) Complex Moduli:{}", q3.get(2, 1));
    println!("    Hodge (1,1) Kähler Moduli: {}", q3.get(1, 1));
    assert!(q3.verify_hodge_symmetry() && q3.euler_characteristic() == -200);

    // 3. Abelian Fourfold (Dimension 4)
    println!("\n[3] Evaluating Abelian 4-fold (Dimension 4):");
    let ab4 = HodgeDiamond::abelian_variety(4);
    println!("    Hodge Symmetry Verified:   {}", ab4.verify_hodge_symmetry());
    println!("    Euler Characteristic χ:    {}", ab4.euler_characteristic());
    println!("    Hodge (2,2) Cycles Space:  {}", ab4.get(2, 2));
    assert!(ab4.verify_hodge_symmetry() && ab4.euler_characteristic() == 0);

    // 4. Riemann-Hodge Bilinear Relations Audit on Primitive Cohomology
    println!("\n[4] Auditing Riemann-Hodge Bilinear Relations on Primitive Cycles:");
    let prim_mat = DMatrix::from_diagonal(&DVector::from_vec(vec![1.0, 1.0, 1.0]));
    let auditor = RiemannHodgeAudit::new(3, prim_mat);
    let sample_prim = DVector::from_vec(vec![1.0, 0.5, 0.2]);
    let (pos, val) = auditor.verify_second_relation_positivity(&sample_prim);
    println!("    Riemann-Hodge Second Relation: strictly positive");
    println!("    Quadratic Norm Value:          {:.6}", val);
    println!("    Positive Definiteness Status:  {}", pos);
    assert!(pos);

    // 5. Riemann Period Matrix Polarized Audit (Siegel Upper Half Space H_g)
    println!("\n[5] Auditing Riemann Period Matrix & Siegel Positivity on Abelian Variety (g=2):");
    use hodge_engine::RiemannPeriodMatrix;
    let tau_sym = DMatrix::from_row_slice(2, 2, &[2.0, 0.5, 0.5, 3.0]);
    let rpm = RiemannPeriodMatrix::new_principally_polarized(2, tau_sym.clone());
    let sym_pass = rpm.verify_riemann_symmetry(1e-12);
    let (pos_pass, det_im) = rpm.verify_riemann_positivity(&tau_sym);
    println!("    Riemann Symmetry Pi * J * Pi^T = 0:    {}", sym_pass);
    println!("    Siegel Positivity Im(tau) > 0 (Det):   {:.6}", det_im);
    println!("    Siegel Metric Polarization Certified:  {}", pos_pass);
    assert!(sym_pass && pos_pass);

    // 6. Siu-Lelong Stratification Audit (Demailly Current Regularization)
    println!("\n[6] Auditing Siu-Lelong Analytic Stratum E_c(T) = {{x in X : nu(T, x) >= c}}:");
    use hodge_engine::SiuLelongCurrentAudit;
    let numbers = vec![0.12, 1.05, 0.45, 2.30, 0.08, 1.00];
    let stratum_auditor = SiuLelongCurrentAudit::new(3, numbers, 1.0);
    let analytic_stratum = stratum_auditor.compute_siu_analytic_stratum();
    println!("    Lelong Multiplicity Threshold c:       1.0");
    println!("    Isolated Analytic Subvarieties Count:  {}", analytic_stratum.len());
    println!("    Certified Discrete Strata Indices:     {:?}", analytic_stratum);
    assert_eq!(analytic_stratum.len(), 3);

    println!("\n================================================================================");
    println!(" [SUCCESS] ALL HODGE TOPOLOGICAL & SPECTRAL INVARIANTS AUDITED AND CERTIFIED!");
    println!("================================================================================");
}

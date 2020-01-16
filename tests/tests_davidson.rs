extern crate approx;
extern crate eigenvalues;
extern crate nalgebra as na;

use approx::relative_eq;
use eigenvalues::algorithms::davidson::EigenDavidson;
use eigenvalues::utils::generate_diagonal_dominant;
use eigenvalues::utils::sort_eigenpairs;

#[test]
fn davidson_eigenvalues() {
    let arr = generate_diagonal_dominant(10, 0.005);
    let eig = sort_eigenpairs(na::linalg::SymmetricEigen::new(arr.clone()));
    
    let dav_eig = EigenDavidson::new(arr.clone(), 2, "DPR").unwrap();
    test_eigenpairs(&eig, dav_eig, 2);
    let dav_eig = EigenDavidson::new(arr.clone(), 2, "GJD").unwrap();
    test_eigenpairs(&eig, dav_eig, 2);
}

fn test_eigenpairs(reference: &na::linalg::SymmetricEigen<f64, na::Dynamic>, dav_eig: EigenDavidson, number: usize) {
    for i in 0..number {
        // Test Eigenvalues
        assert!(relative_eq!(
            reference.eigenvalues[i],
            dav_eig.eigenvalues[i],
            epsilon = 1e-8
        ));
        // Test Eigenvectors
        let x = reference.eigenvectors.column(i);
        let y = dav_eig.eigenvectors.column(i);
        // The autovectors may different in their sign
        // They should be either parallel or antiparallel
        let dot = x.dot(&y).abs();
        assert!(relative_eq!(dot, 1.0, epsilon = 1e-8));
    }
}

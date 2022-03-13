#include "rust/cxx.h"
#include "rust_in_cpp/src/lib.rs.h"
#include <Eigen/Core>
#include <limits>
#include <iostream>


using namespace std;


int main()
{
    constexpr double inf = std::numeric_limits<double>::infinity();

    int m = 7;
    int n = 3;

    std::vector<int> assignment;
    assignment.resize(n);

    Eigen::MatrixXd A(m, n);
    A << -5.69, 5.37, -inf,
        -inf, -3.8, 6.58,
        4.78, -inf, -inf,
        -inf, 5.36, -inf,
        -0.46, -inf, -inf,
        -inf, -0.52, -inf,
        -inf, -inf, -0.60;

    auction_ffi(A.data(), m, n, 1e-3, 10'000, assignment.data());

    for (int t = 0; t < n; t++) {
        cout << "target " << t << " with measurement " << assignment[t] << "\n";
    }
}
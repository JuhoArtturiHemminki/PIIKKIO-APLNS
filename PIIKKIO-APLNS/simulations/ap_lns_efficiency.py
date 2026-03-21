import numpy as np

def calculate_efficiency(temp, delta_i, xi_h, eta_rec):
    k_b = 1.380649e-23
    landauer_limit = k_b * temp * np.log(2)
    e_sigma = (landauer_limit * delta_i) / (xi_h * eta_rec)
    return e_sigma

if __name__ == "__main__":
    t = 300
    di = 1e12
    xh = 0.98
    er = 0.999
    result = calculate_efficiency(t, di, xh, er)
    print(f"{result:.20f}")


import math

def calculate_brillouin(frequency, velocity, n_index):
    theta = math.pi / 2
    shift = (2 * n_index * velocity * frequency * math.sin(theta/2)) / 
299792458
    return shift

if __name__ == "__main__":
    f_uv = 1.407e15
    v_s = 5000
    n = 2.2
    print(calculate_brillouin(f_uv, v_s, n))


import time

def monitor_jitter(target_phi):
    start = time.perf_counter()
    for i in range(1000000):
        _ = (i * target_phi) % (2 * 3.14159)
    end = time.perf_counter()
    return (end - start) / 1000000

if __name__ == "__main__":
    phi = 1.61803398875
    jitter = monitor_jitter(phi)
    print(f"{jitter:.15f}")


import math
import time

from physics import polygon_collision


def make_pentagon(cx, cy, r):
    poly = []
    for i in range(5):
        angle = i * 2 * math.pi / 5
        poly.append(
            (
                cx + r * math.cos(angle),
                cy + r * math.sin(angle),
            )
        )
    return poly


def main():
    poly_a = make_pentagon(0.0, 0.0, 5.0)
    poly_b = make_pentagon(3.0, 2.0, 5.0)

    iterations = 1_000_000
    hits = 0

    start = time.perf_counter()

    for i in range(iterations):
        offset = math.sin(i * 0.001) * 0.01

        moved_b = [(x + offset, y - offset) for (x, y) in poly_b]

        if polygon_collision(poly_a, moved_b):
            hits += 1

    end = time.perf_counter()

    duration = end - start
    ms = duration * 1000
    us_per_call = (duration * 1_000_000) / iterations

    print("Iterations:", iterations)
    print("Hits:", hits)
    print(f"Time: {ms:.2f} ms")
    print(f"Avg per check: {us_per_call:.3f} µs")


if __name__ == "__main__":
    main()

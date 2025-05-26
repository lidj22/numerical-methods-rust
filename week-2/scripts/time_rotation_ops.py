

# we'll just avoid all the abstractions.

import time
import numpy


def main():
    state = numpy.array([
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.]
    ])

    start = time.time()
    for i in range(1000002):
        state = numpy.matmul(
            numpy.array([
                [1., 0., 0.],
                [0., 0., -1.],
                [0., 1., 0.]
            ]),
            state
        )
    duration = time.time() - start
    print(f"Finished calculation. Time = {duration} seconds.")

    # takes around 1.5 seconds.
    print("Final state is:")
    for i in range(3):
        print(state[i])
    
    # now do one with vanilla python.
    state = [
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.]
    ]

    start = time.time()
    for i in range(1000002):
        rotation = [
            [1., 0., 0.],
            [0., 0., -1.],
            [0., 1., 0.]
        ]
        next_state = [
            [0., 0., 0.],
            [0., 0., 0.],
            [0., 0., 0.],
        ]
        for j in range(3):
            for k in range(3):
                for m in range(3):
                    next_state[j][k] += state[j][m] * rotation[m][k]
        state = next_state
    duration = time.time() - start
    print(f"Finished calculation. Time = {duration} seconds.")

    # takes around 2.3 seconds?
    print("Final state is:")
    for i in range(3):
        print(state[i])

if __name__ == "__main__":
    main()


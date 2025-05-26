import time
import numpy as np

def main():
    # Pre-allocate the rotation matrix once outside the loop
    rotation = np.array([
        [1., 0., 0.],
        [0., 0., -1.],
        [0., 1., 0.]
    ], dtype=np.float64)
    
    # Create initial state
    state = np.eye(3, dtype=np.float64)
    
    # Use numpy's faster dot function instead of matmul
    start = time.time()
    for _ in range(1000002):
        # In-place matrix multiplication using np.dot is faster
        # But NumPy doesn't allow direct in-place operations for matrix mult
        # So we'll use a temporary variable
        state = np.dot(rotation, state)
    
    duration = time.time() - start
    print(f"Optimized NumPy calculation. Time = {duration} seconds.")
    
    print("Final state is:")
    for i in range(3):
        print(state[i])
    
    # Even faster: pre-compute powers of the matrix for larger steps
    # This uses the mathematical property that M^(2^k) = (M^(2^(k-1)))^2
    # We can compute the result in O(log n) steps instead of O(n)
    state = np.eye(3, dtype=np.float64)
    temp_rot = rotation.copy()
    steps = 1000002
    
    start = time.time()
    # Binary exponentiation algorithm
    while steps > 0:
        if steps % 2 == 1:
            state = np.dot(temp_rot, state)
        temp_rot = np.dot(temp_rot, temp_rot)
        steps //= 2
        
    duration = time.time() - start
    print(f"Binary exponentiation NumPy calculation. Time = {duration} seconds.")
    
    print("Final state is:")
    for i in range(3):
        print(state[i])
    
if __name__ == "__main__":
    main() 
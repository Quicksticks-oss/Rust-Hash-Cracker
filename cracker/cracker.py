import hashlib
import time

def generate_combinations(alphabet, current, length, hash_base, hash_dict):
    if length == 0:
        string = ''.join(current)
        if string in hash_dict:
            if hash_dict[string] == hash_base:
                print(f"Cracked: {string}")
                return True
    else:
        for char in alphabet:
            current.append(char)
            result = generate_combinations(alphabet, current, length - 1, hash_base, hash_dict)
            if result:
                return True
            current.pop()
    return False

def precompute_hashes(alphabet, max_length):
    hash_dict = {}
    for length in range(1, max_length + 1):
        for char in alphabet:
            generate_combinations(alphabet, [char], length - 1, None, hash_dict)
    return hash_dict

def main():
    crackme = "532eaabd9574880dbf76b9b8cc00832c20a6ec113d682299550d7a6e0f345e25"

    alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!."
    base = len(alphabet)
    max_length = 12

    start_time = time.time()

    hash_dict = precompute_hashes(alphabet, max_length)

    for length in range(1, max_length + 1):
        result = generate_combinations(alphabet, [], length, crackme, hash_dict)
        if result:
            break

    end_time = time.time()
    elapsed_time = end_time - start_time
    print(f"Elapsed Time: {elapsed_time}")

if __name__ == "__main__":
    main()

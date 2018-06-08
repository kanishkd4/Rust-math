from math import gcd
import numpy as np

def square_add_mod(x, n, c=2):
    return (x**2+c)%n

def pollard_rho(n, x, c):
    """"
    https://www.cs.colorado.edu/~srirams/courses/csci2824-spr14/pollardsRho.html
    https://www.geeksforgeeks.org/pollards-rho-algorithm-prime-factorization/
    """
    y = square_add_mod(x, n, c)
    p = gcd(abs(y-x), n)
    if p == 1:
        while p == 1:
            x = square_add_mod(x, n, c)
            y = square_add_mod(square_add_mod(y, n, c), n, c)
            p = gcd(abs(y-x), n)
            if p > 1:
                if p == n:
                    print("If n is not prime, start with a different random value")
                    break
    return p

def prime_check(n):
    flag = 1
    if n in [2, 3]:
        flag = 1
    else:
        if ((n+1)%6 != 0) & ((n-1)%6 != 0):
            flag = 0
        else:
            for i in range(2, np.int(np.floor(np.sqrt(n)))+1):
                if n%i == 0:
                    flag = 0
                    break
    flag = np.where(n in [2, 3], 1 ,flag)
    return flag

def prime_seq(rank=False, to=False):
    i = 3
    n = 5
    if rank is not False:
        a = a = [0]*10
        a[0], a[1] = 2, 3
        while i < rank:
            if prime_check(n) == 1:
                a[i+1] = n
                i += 1
            n += 2
    else:
        n = 3
        i = 2
        a = [2]
        while n < to:
            if prime_check(n) == 1:
                a.append(n)
                i += 1
            n += 2
    return a
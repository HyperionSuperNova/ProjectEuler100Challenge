import math
'''
Multiples of 3 and 5
   
Problem 1
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
'''


# Solution 1.1 (iterative)

# Approximative Complexity : O(n)
def solutionBrute():
    suma = 0
    for i in range(1000):
        if i % 3 == 0 or i % 5 == 0:
            suma += i
    return suma


# Solution 1.2 (functionnal)

lambdaBrute = sum(filter(lambda num: num %
                         5 == 0 or num % 3 == 0, range(1000)))

# Solution 2

# Approximative Complexity : O(n/3+n/15+n/5) = O(9n/15)


def solutionOpti1():
    suma = 0
    for i in range(math.ceil(1000/3)):
        if 3*i % 5 != 0:
            suma += 3*i
    for i in range((1000//5)):
        if 5*i % 3 != 0:
            suma += 5*i
    for i in range(math.ceil(1000/15)):
        suma = suma + 15*i
    return suma


if __name__ == "__main__":
    print(solutionBrute())
    print("-------------------")
    print(solutionOpti1())
    print("-------------------")
    print(lambdaBrute)

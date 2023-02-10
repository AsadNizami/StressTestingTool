from sys import stdin
input = stdin.buffer.readline

def solution(num):
    i = 2

    while i*i <= num:
        if num % i == 0:
            return False
        i += 1

    return True


n = int(input())
solution(n)

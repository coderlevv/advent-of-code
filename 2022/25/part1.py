#!/bin/python3
def snafu2dec(s):
    """Convert SNAFU to decimal.

    Parameters
    ----------
    s : string
        String representation of SNAFU number.

    Returns
    -------
    int
        Decimal number corresponding to the SNAFU number.
    """
    res = 0
    for i in range(len(s)):
        d = s[i] # digit
        if d == '-':
            v = -1
        elif d == '=':
            v = -2
        else:
            v = int(d)
        res += v * (5**(len(s)-1-i))
    return res


def expand(d, k):
    """Expand snafu digit >2 to a representation with 1,2,- and =.
    
    Parameters
    ----------
    d : integer
        SNAFU digit at postions k.
    k : integer
        Position of the digit. Right-to-left zero-indexed.

    Returns
    -------
    string
        A string representation of a SNAFU digit >2.
    """

    dig = ['-', '=']
    zero_tail = '0' * k
    n = int(d) * 5**k
    expanded = [i + ''.join(d) + zero_tail for i in ['1', '2'] for d in dig]
    res = [snafu2dec(e) for e in expanded]
    idx = res.index(n)
    return expanded[idx].replace('0', '')

def base5(n):
    """Convert decimal in base 5 number.
    
    Parameters
    ----------
    n : integer
        Decimal to convert.

    Returns
    -------
    string
        Base 5 representation of decimal n.
    """

    res = []
    i = 0
    # find leftmost digit position
    while 5**i < n:
        i += 1
    for k in range(i-1,-1,-1):
        d = n // 5**k
        res.append(n // 5**k)
        n -= d * 5**k
    return ''.join([str(d) for d in res])


if __name__ == '__main__':
    with open("input", "r") as f:
        content = f.read()
    num = []
    for r in content.split("\n"):
        if r.strip() != "":
            num.append(r.strip())

    dec = [snafu2dec(s) for s in num]
    total = sum(dec)

    # first convert total in base 5 representation
    q = base5(total)
    # SNAFU is found by expanding any digit >2 in the 
    # base 5 number into it's SNAFU representation 
    # using 1,2,- & =, making sure that resulting
    # carry over is added to subsequent digits.
    res = []
    carry = 0
    for i in range(len(q)):
        d = q[len(q)-1-i]
        dc = int(d) + carry
        if dc > 4:
            #res.append(str(dc % 5)) 
            dc = dc % 5
            res.append(str(dc))
            carry = 1
        elif dc in [0, 1, 2]:
            res.append(str(dc))
            carry = 0
        else:
            s = expand(dc, i)
            res.append(s[1:])
            carry = int(s[0])
    res = ''.join(res[::-1])
    print(res)
    # check result
    print(snafu2dec(res)==total)

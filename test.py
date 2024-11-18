import sympy


with open("pi_2.txt","w") as f:
    f.write(str(sympy.pi.evalf(1000000)))
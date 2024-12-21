import io
import re

res = 0
with io.open("day1.txt", "r") as f:
    lines = f.readlines()
for line in lines:
    line = re.sub(r"\D", "", line)
    res += int(line[0] + line[-1])
print(res)


res = 0
with io.open("day1.txt", "r") as f:
    lines = f.readlines()
digmap = dict(one=1, two=2, three=3, four=4, five=5, six=6, seven=7, eight=8, nine=9)
kk = (
    r"(?=("
    + "|".join([str(v) for v in digmap.values()])
    + "|"
    + "|".join(digmap.keys())
    + "))"
)
print(kk)


def conv(x):
    try:
        v = int(x)
    except ValueError:
        v = digmap[x]
    return str(v)


for line in lines:
    line = line.strip()
    print(line, end="")
    m = re.findall(kk, line)
    print("--> ", m)
    res += int(conv(m[0]) + conv(m[-1]))
print(res)

import re

with open("input", 'r') as f:
    content = f.read()
    index = 0
    while index < len(content):
        if content[index:index+7] == "don't()":
            index += 7
            while content[index:index+4] != "do()":
                a = list(content)
                a[index] = "x"
                content = "".join(a)
                index += 1
        else:
            index += 1
    print(content)
    acc = 0
    muls = re.findall(r"mul\(\d+\,\d+\)", content)
    print(len(muls))
    for mul in muls:
        tmp = 1
        for i in re.findall(r"\d+", mul):
            tmp *= int(i)
        acc += tmp
    print(acc)

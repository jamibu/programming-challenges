filename = 'example.txt'

with open(filename, 'r') as f:
    lines = f.readlines()

lines = [l.split() for l in lines]

for line in lines:
    diff = [int(line[i+1]) - int(line[i]) for i in range(0, len(lines) - 2)]
    increase = [int(line[i]) > int(line[i+1]) for i in range(0, len(lines) - 2)]

    print(line)
    print('  - diffs:', diff)
    print('  - increasing:', all(inc == True for inc in increase))
    print('  - decreasing:', all(inc == False for inc in increase))

def lmap(f, l):
    return list(map(f, l))


f = "input"

with open(f) as r:
    s = r.read().rstrip()
    lgroups = s.split("\n\n")

presents = lgroups[:-1]
density = [present.count("#") for present in presents]

regions = lgroups[-1].split("\n")
NO, Maybe, Definitely = range(3)
results = []
for region in regions:
    xy, counts = region.split(": ")
    counts = lmap(int, counts.split(" "))
    x, y = lmap(int, xy.split("x"))
    min_space = sum(a * b for a, b in zip(counts, density))
    total_presents = sum(counts)
    if min_space > x * y:
        results.append(NO)
    elif total_presents <= (x // 3) * (y // 3):  # can fit each in its own 3x3
        results.append(Definitely)
    else:
        results.append(Maybe)
print(f"{results.count(NO)=}")
print(f"{results.count(Definitely)=}")
print(f"{results.count(Maybe)=}")

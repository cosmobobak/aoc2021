
with open("src/tasks/task1.txt") as f:
    data = f.readlines()
    data = [int(x.strip()) for x in data]

t1 = sum(a < b for a, b in zip(data, data[1:]))
print(f"Task 1: {t1}")

windowed = [sum(data[i:i+3]) for i in range(len(data) - 2)]
t2 = sum(a < b for a, b in zip(windowed, windowed[1:]))
print(f"Task 2: {t2}")
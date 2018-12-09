file = open('day1-data.txt', 'r')
data = [int(line) for line in file]
print(sum(data))

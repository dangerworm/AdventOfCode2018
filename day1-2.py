from time import sleep

##file = [1,2,3,-6]

file = open('day1-data.txt', 'r')
data = [int(line) for line in file]
##print(sum(data))

frequencies = []
frequency = 0
counter = 0

while frequency not in frequencies:
    frequencies.append(frequency)
    frequency += data[counter]

    counter += 1
    if counter == len(data):
        counter = 0

    ##print(frequencies)
    ##print(frequency)
    ##print(counter)

    ##sleep(0.5)

print(frequency)

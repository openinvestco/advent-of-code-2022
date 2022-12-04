import re

def main():
    part_one()
    part_two()

def loop_over_pairs(filename, closure):
    total = 0
    with open(filename, "r", encoding = 'utf-8') as file:
        for line in file:
            re_pair = re.compile(r'(\d+)-' + r'(\d+)')
            t1, t2 = re.findall(re_pair, line)
            if closure(t1, t2) or closure(t2, t1):
                total += 1
    return total

def part_one():
    contains = lambda t1, t2 : int(t1[0]) <= int(t2[0]) and int(t1[1]) >= int(t2[1])
    print(loop_over_pairs("../input.txt", contains))
    
def part_two():
    overlaps = lambda t1, t2 : int(t1[0]) <= int(t2[0]) and int(t1[1]) >= int(t2[0]) or int(t1[0]) <= int(t2[1]) and int(t1[1]) >= int(t2[1])
    print(loop_over_pairs("../input.txt", overlaps))



if __name__ == '__main__':
    main()
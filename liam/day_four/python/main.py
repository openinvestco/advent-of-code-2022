import re

def main():
    part_one()
    part_two()

def part_one():
    total_containing = 0
    with open("../input.txt", "r", encoding = 'utf-8') as file:
        for line in file:
            re_pair = re.compile(r'(\d+)-' + r'(\d+)')
            t1, t2 = re.findall(re_pair, line)
            contains = lambda t1, t2 : int(t1[0]) <= int(t2[0]) and int(t1[1]) >= int(t2[1])
            if contains(t1, t2) or contains(t2, t1):
                total_containing += 1
    print(total_containing)       
    
def part_two():
    total_containing = 0
    with open("../input.txt", "r", encoding = 'utf-8') as file:
        for line in file:
            re_pair = re.compile(r'(\d+)-' + r'(\d+)')
            t1, t2 = re.findall(re_pair, line)
            overlaps = lambda t1, t2 : int(t1[0]) <= int(t2[0]) and int(t1[1]) >= int(t2[0]) or int(t1[0]) <= int(t2[1]) and int(t1[1]) >= int(t2[1])
            if overlaps(t1, t2) or overlaps(t2, t1):
                total_containing += 1
    print(total_containing)      


if __name__ == '__main__':
    main()
def find_hits(line):
    parts = [s.strip() for s in line.split(":")[1].split('|')]
    numbers = [s.strip() for s in parts[0].split(' ')]
    game_numbers = [s.strip() for s in parts[1].split(' ')]

    return sum([1 for g in game_numbers if g in numbers])


def calc_first(line):
    count_hits = find_hits(line)
    if count_hits == 0:
        return 0

    final_val = 1
    if count_hits > 1:
        for _ in range(1, count_hits):
            final_val *= 2
    return final_val


def calc_second(line, save_before_hits):
    count_hits = find_hits(line)
    return count_hits * save_before_hits


def main():
    path = 'day04.txt'
    first_sum, second_sum = 0, 0
    with open(path, 'r') as file:
        for line in file:
            line = line.strip()
            first_sum += calc_first(line)
    print(f"Part 1: {first_sum}")
    print(f"Part 2: {second_sum}")


if __name__ == '__main__':
    main()

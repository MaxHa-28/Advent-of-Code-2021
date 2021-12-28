def get_number(data, most_common):
    for n in range(12):
        count = sum(d[n] == '1' for d in data)

        if most_common:
            search_val = '1' if count >= len(data) / 2 else '0'
        else:
            search_val = '0' if count >= len(data) / 2 else '1'

        if len(data) > 1:
            data = [d for d in data if d[n] == search_val]

    return data[0]

if __name__ == '__main__':
    with open("data.txt") as f:
        data = [line.strip() for line in f.readlines()]
        oxygen = get_number(data, True)
        co2 = get_number(data, False)
        print(int(oxygen, 2) * int(co2, 2))
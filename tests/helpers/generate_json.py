import argparse
import csv
import json

def generate_json(csv_name: str, seed_val: str):
    field_names = (
        "name",
        "elem1_offset",
        "elem2_offset",
        "elem3_offset",
        "elem4_offset",
        "elem5_offset",
        "elem6_offset",
        "elem7_offset",
        "elem8_offset",
        "elem9_offset",
        "elem10_offset",
        "elem11_offset",
        "elem12_offset",
        "elem13_offset",
        "elem14_offset",
        "elem15_offset",
        "elem16_offset",
        "elem17_offset",
        "elem18_offset",
        "elem19_offset",
        "elem20_offset",
        "elem21_offset",
        "elem22_offset",
        "elem23_offset",
        "elem24_offset",
        "elem25_offset",
        "elem26_offset",
        "elem27_offset",
        "elem28_offset",
        "elem29_offset",
        "elem30_offset",
        "elem31_offset",
        "elem32_offset",
        "elem33_offset",
        "elem34_offset",
        "elem35_offset",
        "elem36_offset",
        "elem37_offset",
        "elem38_offset",
        "elem39_offset",
        "elem40_offset",
        "elem41_offset",
        "elem42_offset",
        "elem43_offset",
        "elem44_offset",
        "elem45_offset",
        "elem46_offset",
        "elem47_offset",
        "elem48_offset",
        "elem49_offset",
        "elem50_offset")

    reader = csv.DictReader(open(csv_name, 'r'), field_names)

    result = {}

    for row in reader:
        row_name = row['name']
        result[row_name] = {}
        row.pop('name', None)
        for elem in row:
            if elem != None and row[elem] != None and row[elem] != '':
                result[row_name][elem] = int(row[elem])

    print(json.dumps({seed_val: result}, indent=4))

if __name__ == '__main__':
    parser = argparse.ArgumentParser()

    parser.add_argument('--csv', type=str, required=True)
    parser.add_argument('--seed', type=str, required=True)

    args = parser.parse_args()
    generate_json(args.csv, args.seed)

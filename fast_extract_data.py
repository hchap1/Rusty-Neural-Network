from random import randint
variables = {}
names = []
name_map = {}

targets = []
target_value = "marital-status"
target_map = []

# Read and process the data file
with open("adult\\adult.data", "r") as data:
    lines = [
        x.strip().rstrip(".").lower().split(", ") for x in data.readlines() if x.strip()
    ]

# Read and process the names file
with open("adult\\adult.names", "r") as data:
    for line in data:
        name, vals = line.strip().rstrip(".").lower().split(": ")
        vals = vals.split(", ")
        if name != target_value:
            variables[name] = vals
            names.append(name)
            for v in vals:
                name_map[v] = name
        else:
            target_map.extend(vals)

var_classes = []
for name in names:
    var_classes.append(", ".join(variables[name]))
var_classes.append(", ".join(target_map))
new_lines = []

# Process each line and map to numeric values
for l in lines:
    numeric = []
    skip = False
    for idx, field in enumerate(l):
        # Map field to variable or target
        n = name_map.get(field)
        if n is not None:
            val = variables[n].index(field)
        elif field in target_map:
            if "?" in l:
                skip = True
                break
            targets.append(target_map.index(field))
            continue
        elif field == "?":
            skip = True
            break
        else:
            val = float(field)

        numeric.append(val)

    if not skip and randint(0, 100) < 5:
        new_lines.append(numeric)

# Write processed data to the file
with open("census.td", "w") as data:
    data.writelines(
        f"{', '.join(map(str, line))} | {targets[idx]}\n"
        for idx, line in enumerate(new_lines)
    )
with open("census.cl", "w") as data:
    data.write("\n".join(var_classes))

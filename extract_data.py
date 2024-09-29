variables = {}
names = []
name_map = {}

targets = []
target_value = "marital-status"
target_map = []

with open("adult\\adult.data", "r") as data:
    lines = [x.strip("\n").strip(".").lower().split(", ") if x.strip("\n") != '' else None for x in data.readlines()]

with open("adult\\adult.names", "r") as data:
    data = [x.strip("\n").strip(".").lower().split(": ") for x in data.readlines()]
    for name, vals in data:
        if name != target_value:
            variables[name] = vals.split(", ")
            names.append(name)
            for v in variables[name]:
                name_map[v] = name
        else:
            for val in vals.split(", "): target_map.append(val)

new_lines = []

for l in lines:
    if l != None:
        numeric = []
        skip = False
        for idx, field in enumerate(l):
            n = name_map.get(field)
            val = field
            if n != None:
                val = variables[n].index(field)
            elif field in target_map:
                if "?" in l:
                    skip = True
                    break
                else:
                    targets.append(target_map.index(val))
                    continue
            if val == "?":
                skip = True
                break
            else:
                val = float(val)
            numeric.append(val)
        if skip:
            continue
        new_lines.append(", ".join([str(x) for x in numeric]))

with open("census.td", "w") as data:
    final_string = ""
    for n in range(len(new_lines)):
        final_string += str(", ".join(new_lines[n]))
        final_string += " | "
        final_string += str(targets[n])
        final_string += "\n"
    data.write(final_string)
